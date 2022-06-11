use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    path::PathBuf,
};

use fs_extra::{dir::copy as copy_directory, file::copy as copy_file};
use owo_colors::OwoColorize;
use remove_dir_all::remove_dir_all;
use semver::Version;
use serde::{Deserialize, Serialize};

use super::package::SharedPackageConfig;
use crate::data::{config::Config, package::PackageConfig};

// TODO: Somehow make a global singleton of sorts/cached instance to share across places
// like resolver
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FileRepository {
    pub artifacts: HashMap<String, HashMap<Version, SharedPackageConfig>>,
}

impl FileRepository {
    pub fn get_artifacts_from_id(
        &self,
        id: &str,
    ) -> Option<&HashMap<Version, SharedPackageConfig>> {
        self.artifacts.get(id)
    }

    pub fn get_artifact(&self, id: &str, version: &Version) -> Option<&SharedPackageConfig> {
        match self.artifacts.get(id) {
            Some(artifacts) => artifacts.get(version),
            None => None,
        }
    }

    pub fn add_artifact(
        &mut self,
        package: SharedPackageConfig,
        project_folder: PathBuf,
        binary_path: Option<PathBuf>,
        debug_binary_path: Option<PathBuf>,
    ) {
        if !self.artifacts.contains_key(&package.config.info.id) {
            self.artifacts
                .insert(package.config.info.id.clone(), HashMap::new());
        }

        Self::add_to_cache(&package, project_folder, binary_path, debug_binary_path);

        let id_artifacts = self.artifacts.get_mut(&package.config.info.id).unwrap();

        id_artifacts.insert(package.config.info.version.clone(), package);
    }

    fn copy_to_cache(a: &PathBuf, b: &PathBuf) {
        if a.is_dir() {
            fs::create_dir_all(&b)
                .unwrap_or_else(|e| panic!("Failed to create {b:?} path. Cause {e:?}"));
        } else {
            let parent = b.parent().unwrap();
            fs::create_dir_all(parent)
                .unwrap_or_else(|e| panic!("Failed to create {parent:?} path. Cause {e:?}"));
        }

        let result = if a.is_dir() {
            let mut options = fs_extra::dir::CopyOptions::new();
            options.overwrite = true;
            options.copy_inside = true;
            options.content_only = true;
            // copy it over
            copy_directory(a, b, &options)
        } else {
            // if it's a file, copy that over instead
            let mut options = fs_extra::file::CopyOptions::new();
            options.overwrite = true;
            copy_file(a, b, &options)
        };

        result.unwrap_or_else(|e| panic!("Unable to copy from {:?} to {:?}. Cause {e:?}", a, b));
    }

    fn add_to_cache(
        package: &SharedPackageConfig,
        project_folder: PathBuf,
        binary_path: Option<PathBuf>,
        debug_binary_path: Option<PathBuf>,
    ) {
        println!(
            "Adding cache for local dependency {} {}",
            package.config.info.id.bright_red(),
            package.config.info.version.bright_green()
        );
        let config = Config::read_combine();
        let cache_path = config
            .cache
            .unwrap()
            .join(&package.config.info.id)
            .join(package.config.info.version.to_string());

        let src_path = cache_path.join("src");

        let tmp_path = cache_path.join("tmp");

        // Downloads the repo / zip file into src folder w/ subfolder taken into account

        // if the tmp path exists, but src doesn't, that's a failed cache, delete it and try again!
        if tmp_path.exists() {
            remove_dir_all(&tmp_path).expect("Failed to remove existing tmp folder");
        }

        if src_path.exists() {
            remove_dir_all(&src_path).expect("Failed to remove existing src folder");
        }

        fs::create_dir_all(&src_path).expect("Failed to create lib path");

        if binary_path.is_some() || debug_binary_path.is_some() {
            let lib_path = cache_path.join("lib");
            let so_path = lib_path.join(package.config.get_so_name());
            let debug_so_path = lib_path.join(format!("debug_{}", package.config.get_so_name()));

            if let Some(binary_path_unwrapped) = &binary_path {
                Self::copy_to_cache(binary_path_unwrapped, &so_path);
            }

            if let Some(debug_binary_path_unwrapped) = &debug_binary_path {
                Self::copy_to_cache(debug_binary_path_unwrapped, &debug_so_path);
            }
        }

        let original_shared_path = project_folder.join(&package.config.shared_dir);
        let original_package_file_path = project_folder.join("qpm.json");

        Self::copy_to_cache(
            &original_shared_path,
            &src_path.join(&package.config.shared_dir),
        );
        Self::copy_to_cache(&original_package_file_path, &src_path.join("qpm.json"));

        let package_path = src_path.join("qpm.json");
        let downloaded_package = PackageConfig::read_path(package_path);

        // check if downloaded config is the same version as expected, if not, panic
        if downloaded_package.info.version != package.config.info.version {
            panic!(
                "Downloaded package ({}) version ({}) does not match expected version ({})!",
                package.config.info.id.bright_red(),
                downloaded_package.info.version.to_string().bright_green(),
                package.config.info.version.to_string().bright_green(),
            )
        }
    }

    /// always gets the global config
    pub fn read() -> Self {
        let path = Self::global_file_repository_path();
        std::fs::create_dir_all(Self::global_repository_dir())
            .expect("Failed to make config folder");

        if let Ok(mut file) = std::fs::File::open(path) {
            // existed
            let mut config_str = String::new();
            file.read_to_string(&mut config_str)
                .expect("Reading data failed");

            serde_json::from_str::<Self>(&config_str).expect("Deserializing package failed")
        } else {
            // didn't exist
            Self {
                ..Default::default()
            }
        }
    }

    pub fn write(&self) {
        let config = serde_json::to_string_pretty(&self).expect("Serialization failed");
        let path = Self::global_file_repository_path();

        std::fs::create_dir_all(Self::global_repository_dir())
            .expect("Failed to make config folder");
        let mut file = std::fs::File::create(path).expect("create failed");
        file.write_all(config.as_bytes()).expect("write failed");
        println!("Saved local repository Config!");
    }

    pub fn global_file_repository_path() -> PathBuf {
        Self::global_repository_dir().join("qpm.repository.json")
    }

    pub fn global_repository_dir() -> PathBuf {
        dirs::config_dir().unwrap().join("QPM-Rust")
    }
}

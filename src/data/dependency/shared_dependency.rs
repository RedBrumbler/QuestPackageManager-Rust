use std::{
    fs::File,
    io::Cursor,
    path::{Path, PathBuf},
};

use fs_extra::{dir::copy as copy_directory, file::copy as copy_file};
use owo_colors::OwoColorize;
use remove_dir_all::remove_dir_all;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use zip::ZipArchive;

use super::Dependency;
use crate::{
    data::{
        config::Config,
        package::{PackageConfig, SharedPackageConfig},
    },
    utils::{git, network::get_agent},
};

#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SharedDependency {
    pub dependency: Dependency,
    pub version: Version,
}

impl SharedDependency {
    pub fn get_so_name(&self) -> String {
        self.dependency
            .additional_data
            .override_so_name
            .clone()
            .unwrap_or(format!(
                "lib{}_{}.{}",
                self.dependency.id,
                self.version.to_string().replace('.', "_"),
                if self
                    .dependency
                    .additional_data
                    .static_linking
                    .unwrap_or(false)
                {
                    "a"
                } else {
                    "so"
                },
            ))
    }

    pub fn cache(&self, shared_package: &SharedPackageConfig) {
        // Check if already cached
        // if true, don't download repo / header files
        // else cache to tmp folder in package id folder @ cache path
        //          git repo -> git clone w/ or without github token
        //          not git repo (no github.com) -> assume it's a zip
        //          !! HANDLE SUBFOLDER FROM TMP, OR IF NO SUBFOLDER JUST RENAME TMP TO SRC !!
        //          -- now we have the header files --
        // Check if .so files are downloaded, if not:
        // Download release .so and possibly debug .so to libs folder, if from github use token if available
        // Now it should be cached!

        println!(
            "Checking cache for dependency {} {}",
            self.dependency.id.bright_red(),
            self.version.bright_green()
        );
        let config = Config::read_combine();
        let base_path = config
            .cache
            .unwrap()
            .join(&self.dependency.id)
            .join(self.version.to_string());

        let src_path = base_path.join("src");
        let lib_path = base_path.join("lib");
        let tmp_path = base_path.join("tmp");

        let so_path = lib_path.join(shared_package.config.get_so_name());
        let debug_so_path = lib_path.join(format!("debug_{}", shared_package.config.get_so_name()));

        // Downloads the repo / zip file into src folder w/ subfolder taken into account
        if !src_path.exists() {
            // if the tmp path exists, but src doesn't, that's a failed cache, delete it and try again!
            if tmp_path.exists() {
                remove_dir_all(&tmp_path).expect("Failed to remove existing tmp folder");
            }

            // src did not exist, this means that we need to download the repo/zip file from packageconfig.info.url
            std::fs::create_dir_all(src_path.parent().unwrap())
                .expect("Failed to create lib path");
            let url = shared_package.config.info.url.as_ref().unwrap();
            if url.contains("github.com") {
                // github url!
                git::clone(
                    url.clone(),
                    shared_package
                        .config
                        .info
                        .additional_data
                        .branch_name
                        .as_ref(),
                    &tmp_path,
                );
            } else {
                // not a github url, assume it's a zip
                let response = get_agent().get(url).send().unwrap();

                let buffer = Cursor::new(response.bytes().unwrap());
                // Extract to tmp folder
                ZipArchive::new(buffer).unwrap().extract(&tmp_path).unwrap();
            }
            // the only way the above if else would break is if someone put a link to a zip file on github in the url slot
            // if you are reading this and think of doing that so I have to fix this, fuck you

            let from_path =
                if let Some(sub_folder) = &shared_package.config.info.additional_data.sub_folder {
                    // the package exists in a subfolder of the downloaded thing, just move the subfolder to src
                    tmp_path.join(sub_folder)
                } else {
                    // the downloaded thing IS the package, just rename the folder to src
                    tmp_path.clone()
                };

            if from_path.exists() {
                // only log this on debug builds
                #[cfg(debug_assertions)]
                println!(
                    "from: {}\nto: {}",
                    from_path.display().bright_yellow(),
                    src_path.display().bright_yellow()
                );

                if src_path.exists() {
                    let mut line = String::new();
                    println!(
                        "Confirm deletion of folder {}: (y/N)",
                        src_path.display().bright_yellow()
                    );
                    let _ = std::io::stdin().read_line(&mut line).unwrap();
                    if line.starts_with('y') || line.starts_with('Y') {
                        remove_dir_all(&src_path).expect("Failed to remove existing src folder");
                    }
                }
                // HACK: renaming seems to work, idk if it works for actual subfolders?
                std::fs::rename(&from_path, &src_path).expect("Failed to move folder");
            } else {
                panic!("Failed to restore folder for this dependency\nif you have a token configured check if it's still valid\nIf it is, check if you can manually reach the repo");
            }

            // clear up tmp folder if it still exists
            if tmp_path.exists() {
                std::fs::remove_dir_all(tmp_path).expect("Failed to remove tmp folder");
            }
            let package_path = src_path.join("qpm.json");
            let downloaded_package = PackageConfig::read_path(package_path);

            // check if downloaded config is the same version as expected, if not, panic
            if downloaded_package.info.version != self.version {
                panic!(
                    "Downloaded package ({}) version ({}) does not match expected version ({})!",
                    self.dependency.id.bright_red(),
                    downloaded_package.info.version.to_string().bright_green(),
                    self.version.to_string().bright_green(),
                )
            }
        }

        if !lib_path.exists() {
            std::fs::create_dir_all(&lib_path).expect("Failed to create lib path");
            // libs didn't exist or the release object didn't exist, we need to download from packageconfig.info.additional_data.so_link and packageconfig.info.additional_data.debug_so_link
            if !so_path.exists() || File::open(&so_path).is_err() {
                if let Some(so_link) = &shared_package.config.info.additional_data.so_link {
                    // so_link existed, download
                    if so_link.contains("github.com") {
                        // github url!
                        git::get_release(so_link, &so_path);
                    } else {
                        let mut response = get_agent()
                            .get(so_link)
                            .send()
                            .expect("Unable to download so file");

                        // other dl link, assume it's a raw lib file download
                        let mut file =
                            std::fs::File::create(so_path).expect("create so file failed");

                        response
                            .copy_to(&mut file)
                            .expect("Failed to write out downloaded bytes");
                    }
                }
            }

            if !debug_so_path.exists() || File::open(&debug_so_path).is_err() {
                if let Some(debug_so_link) =
                    &shared_package.config.info.additional_data.debug_so_link
                {
                    // debug_so_link existed, download
                    if debug_so_link.contains("github.com") {
                        // github url!
                        git::get_release(debug_so_link, &debug_so_path);
                    } else {
                        // other dl link, assume it's a raw lib file download
                        let mut response = get_agent()
                            .get(debug_so_link)
                            .send()
                            .expect("Unable to download debug so file");

                        let mut file =
                            std::fs::File::create(debug_so_path).expect("create so file failed");

                        response
                            .copy_to(&mut file)
                            .expect("Failed to write out downloaded bytes");
                    }
                }
            }
        }
    }

    pub fn restore_from_cache(&self, also_lib: bool, shared_package: &SharedPackageConfig) {
        // restore from cached files, give error on fail (nonexistent?)
        if Config::read_combine().symlink.unwrap_or(false) {
            self.restore_from_cache_symlink(also_lib, shared_package);
        } else {
            self.restore_from_cache_copy(also_lib, shared_package);
        }
    }

    pub fn collect_to_copy(
        &self,
        also_lib: bool,
        shared_package: &SharedPackageConfig,
    ) -> Vec<(PathBuf, PathBuf)> {
        // TODO: Look into improving the way it gets all the things to copy
        // low priority since this also works
        let config = Config::read_combine();
        let package = PackageConfig::read();

        let base_path = config
            .cache
            .unwrap()
            .join(&self.dependency.id)
            .join(self.version.to_string());
        let src_path = base_path.join("src");
        let libs_path = base_path.join("lib");
        let dependencies_path = Path::new(&package.dependencies_dir);
        std::fs::create_dir_all(dependencies_path).unwrap();
        let dependencies_path = dependencies_path.canonicalize().unwrap().join("includes");
        let local_path = dependencies_path.join(&self.dependency.id);
        let mut to_copy = Vec::new();
        if also_lib {
            let use_release = if self.dependency.additional_data.use_release.is_some() {
                self.dependency.additional_data.use_release.unwrap()
            } else if let Some(local_dep) = package
                .dependencies
                .iter()
                .find(|el| el.id == self.dependency.id)
            {
                local_dep.additional_data.use_release.unwrap_or(false)
            } else {
                false
            };

            let prefix = if !use_release { "debug_" } else { "" };

            let suffix = if let Some(override_so_name) = shared_package
                .config
                .info
                .additional_data
                .override_so_name
                .clone()
            {
                override_so_name
            } else {
                format!(
                    "lib{}_{}.so",
                    self.dependency.id,
                    self.version.to_string().replace('.', "_")
                )
            };

            let mut so_name: String = format!("{}{}", prefix, suffix);

            // if not headers only, copy over .so file
            if shared_package
                .config
                .info
                .additional_data
                .headers_only
                .is_none()
                || !shared_package
                    .config
                    .info
                    .additional_data
                    .headers_only
                    .unwrap()
            {
                let mut lib_so_path = libs_path.join(&so_name);
                // if it doesn't exist, use it without debug
                if !lib_so_path.exists() {
                    #[cfg(debug_assertions)]
                    println!(
                        "Path {} did not exist, editing to remove debug_",
                        lib_so_path.display().bright_yellow()
                    );

                    so_name = so_name.replace("debug_", "");
                    lib_so_path = libs_path.join(&so_name);
                }

                let local_so_path = Path::new(&package.dependencies_dir)
                    .canonicalize()
                    .unwrap()
                    .join("libs")
                    .join(&so_name.replace("debug_", ""));
                // from to
                to_copy.push((lib_so_path, local_so_path));
            }
        }
        // copy  shared / include over
        let cache_shared_path = src_path.join(&shared_package.config.shared_dir);
        let shared_path = local_path.join(&shared_package.config.shared_dir);
        to_copy.push((cache_shared_path, shared_path));

        if let Some(extra_files) = &self.dependency.additional_data.extra_files {
            for entry in extra_files.iter() {
                let cache_entry_path = src_path.join(entry);
                let entry_path = local_path.join(entry);
                to_copy.push((cache_entry_path, entry_path));
            }
        }

        let local_shared_package = SharedPackageConfig::read();
        if let Some(dep) = local_shared_package
            .config
            .dependencies
            .iter()
            .find(|el| el.id == self.dependency.id)
        {
            if let Some(extra_files) = &dep.additional_data.extra_files {
                for entry in extra_files.iter() {
                    let cache_entry_path = src_path.join(entry);
                    let entry_path = local_path.join(entry);
                    to_copy.push((cache_entry_path, entry_path));
                }
            }
        }

        to_copy
    }

    pub fn restore_from_cache_symlink(&self, also_lib: bool, shared_package: &SharedPackageConfig) {
        let to_copy = self.collect_to_copy(also_lib, shared_package);
        // sort out issues with the symlinking, stuff is being symlinked weirdly
        for (from, to) in to_copy.iter() {
            #[cfg(debug_assertions)]
            println!(
                "symlinking\nfrom {}\nto {}",
                from.display().bright_yellow(),
                to.display().bright_yellow()
            );

            // make sure to parent dir exists!
            std::fs::create_dir_all(to.parent().unwrap()).expect("Failed to create parent folder");
            if let Err(e) = symlink::symlink_auto(from, to) {
                #[cfg(windows)]
                println!("Failed to create symlink: {}\nfalling back to copy, did the link already exist, or did you not enable windows dev mode?\nTo disable this warning (and default to copy), use the command {}", e.bright_red(), "qpm config symlink disable".bright_yellow());
                #[cfg(not(windows))]
                println!("Failed to create symlink: {}\nfalling back to copy, did the link already exist?\nTo disable this warning (and default to copy), use the command {}", e.bright_red(), "qpm config symlink disable".bright_yellow());

                if from.is_dir() {
                    let mut options = fs_extra::dir::CopyOptions::new();
                    options.overwrite = true;
                    options.copy_inside = true;
                    options.content_only = true;
                    options.skip_exist = true;
                    copy_directory(from, to, &options).unwrap_or_else(|_| {
                        panic!("Failed to copy directory! From {:#?} To {:#?}", &from, &to)
                    }); // ignore warning, let it raise the error for more details.
                } else if from.is_file() {
                    // we can get the parent beccause this is a file path
                    std::fs::create_dir_all(to.parent().unwrap())
                        .expect("Failed to create containing directory");
                    std::fs::copy(from, to).expect("Failed to copy file!");
                }
            }
        }
    }

    pub fn restore_from_cache_copy(&self, also_lib: bool, shared_package: &SharedPackageConfig) {
        // get the files to copy
        let to_copy = self.collect_to_copy(also_lib, shared_package);
        for (from_str, to_str) in to_copy.iter() {
            let from = Path::new(&from_str);
            let to = Path::new(&to_str);

            #[cfg(debug_assertions)]
            println!(
                "copying\nfrom {}\nto {}",
                from.display().bright_yellow(),
                to.display().bright_yellow()
            );

            // make sure to parent dir exists!
            std::fs::create_dir_all(to.parent().unwrap()).expect("Failed to create parent folder");
            // if dir, make sure it exists
            if !from.exists() {
                println!("The file or folder\n\t'{}'\ndid not exist! what happened to the cache? you should probably run {} to make sure everything is in order...", from.display().bright_yellow(), "qpm cache clear".bright_yellow());
            } else if from.is_dir() {
                std::fs::create_dir_all(to).expect("Failed to create destination folder");
                let mut options = fs_extra::dir::CopyOptions::new();
                options.overwrite = true;
                options.copy_inside = true;
                options.content_only = true;
                // copy it over
                copy_directory(from, to, &options).expect("Failed to copy directory!");
            } else if from.is_file() {
                // if it's a file, copy that over instead
                let mut options = fs_extra::file::CopyOptions::new();
                options.overwrite = true;
                copy_file(from, to, &options).expect("Failed to copy file!");
            }
        }
    }
}

impl From<SharedPackageConfig> for SharedDependency {
    fn from(shared_package: SharedPackageConfig) -> Self {
        let package_config = PackageConfig::read();
        let version_range = if let Some(orig) = package_config
            .dependencies
            .iter()
            .find(|el| el.id == shared_package.config.info.id)
        {
            orig.version_range.clone()
        } else {
            VersionReq::parse(&format!("^{}", shared_package.config.info.version)).unwrap()
        };

        SharedDependency {
            dependency: Dependency {
                id: shared_package.config.info.id.to_string(),
                version_range,
                additional_data: shared_package.config.info.additional_data,
            },
            version: shared_package.config.info.version,
        }
    }
}

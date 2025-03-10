use std::{io::Write, path::PathBuf};

use clap::Args;

use crate::data::{
    config::Config,
    file_repository::FileRepository,
    package::{PackageConfig, SharedPackageConfig},
};

#[derive(Args, Debug, Clone)]
pub struct InstallOperation {
    pub binary_path: Option<PathBuf>,
    pub debug_binary_path: Option<PathBuf>,

    #[clap(long)]
    pub cmake_build: Option<bool>,
    // pub additional_folders: Vec<String> // todo
}

pub fn execute_install_operation(install: InstallOperation) {
    println!("Publishing package to local file repository");
    let package = PackageConfig::read();
    let shared_package = SharedPackageConfig::from_package(&package);

    // create used dirs
    std::fs::create_dir_all("src").expect("Failed to create directory");
    std::fs::create_dir_all("include").expect("Failed to create directory");
    std::fs::create_dir_all(&shared_package.config.shared_dir).expect("Failed to create directory");

    // write the ndk path to a file if available
    let config = Config::read_combine();
    if let Some(ndk_path) = config.ndk_path {
        let mut file = std::fs::File::create("ndkpath.txt").expect("Failed to create ndkpath.txt");
        file.write_all(ndk_path.as_bytes())
            .expect("Failed to write out ndkpath.txt");
    }

    shared_package.write();

    let mut binary_path = install.binary_path;
    let mut debug_binary_path = install.debug_binary_path;

    let header_only = package.info.additional_data.headers_only.unwrap_or(false);
    #[cfg(debug_assertions)]
    println!("Header only: {}", header_only);

    if !header_only {
        if binary_path.is_none() && install.cmake_build.unwrap_or(true) {
            binary_path = Some(
                PathBuf::from(format!("./build/{}", shared_package.config.get_so_name()))
                    .canonicalize()
                    .unwrap(),
            );
        }

        if debug_binary_path.is_none() && install.cmake_build.unwrap_or(true) {
            debug_binary_path = Some(
                PathBuf::from(format!(
                    "./build/debug/{}",
                    shared_package.config.get_so_name()
                ))
                .canonicalize()
                .unwrap(),
            );
        }
    }

    if let Some(p) = &debug_binary_path {
        if !p.exists() {
            println!("Could not find debug binary {p:?}, skipping")
        }
    }

    if let Some(p) = &binary_path {
        if !p.exists() {
            println!("Could not find binary {p:?}, skipping")
        }
    }

    let mut repo = FileRepository::read();
    repo.add_artifact(
        shared_package,
        PathBuf::from(".")
            .canonicalize()
            .expect("Unable to canocalize path"),
        binary_path,
        debug_binary_path,
    );
    repo.write();
}

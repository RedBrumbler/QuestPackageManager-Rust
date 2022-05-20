use std::{io::Write, path::PathBuf};

use clap::Args;

use crate::data::{
    config::Config,
    package::{PackageConfig, SharedPackageConfig}, file_repository::FileRepository,
};


#[derive(Args, Debug, Clone)]
pub struct InstallOperation {
    pub binary_path: Option<PathBuf>

    // pub additional_folders: Vec<String> // todo

}

pub fn execute_install_operation(install: InstallOperation) {
    println!("package should be restoring");
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

    FileRepository::read().add_artifact(shared_package, PathBuf::from(".").canonicalize().expect("Unable to canocalize path"), install.binary_path)

}

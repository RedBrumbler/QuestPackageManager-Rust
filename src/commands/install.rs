use std::{io::Write, path::PathBuf};

use clap::Args;

use crate::data::{
    config::Config,
    package::{PackageConfig, SharedPackageConfig}, file_repository::FileRepository,
};


#[derive(Args, Debug, Clone)]
pub struct InstallOperation {
    pub binary_path: Option<PathBuf>,

    #[clap(long, short)]
    pub cmake_build: Option<bool>
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

    if binary_path.is_none() && install.cmake_build.unwrap_or(true) {
        binary_path = Some(PathBuf::from(format!("./build/{}", shared_package.config.get_so_name())).canonicalize().unwrap());
    }

    let mut repo = FileRepository::read();
    repo.add_artifact(shared_package, PathBuf::from(".").canonicalize().expect("Unable to canocalize path"), binary_path);
    repo.write();
}

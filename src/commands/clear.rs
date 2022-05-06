use owo_colors::OwoColorize;
use remove_dir_all::remove_dir_all;
use walkdir::WalkDir;

use crate::data::package::PackageConfig;

pub fn execute_clear_operation() {
    remove_dependencies_dir();
    std::fs::remove_file("qpm.shared.json").ok();
    std::fs::remove_file("extern.cmake").ok();
    std::fs::remove_file("qpm_defines.cmake").ok();
    std::fs::remove_file("mod.json").ok();
}

pub fn remove_dependencies_dir() {
    let package = PackageConfig::read();
    let extern_path = std::path::Path::new(&package.dependencies_dir);

    let current_path = std::path::Path::new("");

    // If extern is "" or ".." etc. or is a path that is an
    // ancestor of the current directory, fail fast
    if current_path.canonicalize().unwrap() == extern_path.canonicalize().unwrap()
        || current_path
            .ancestors()
            .any(|path| path.canonicalize().unwrap() == extern_path.canonicalize().unwrap())
    {
        panic!(
            "Current path {:?} would be deleted since extern path {:?} is an ancestor or empty",
            current_path, extern_path
        );
    }

    if extern_path.exists() {
        for entry in WalkDir::new(extern_path.canonicalize().unwrap()).min_depth(1) {
            let path = entry.unwrap().into_path();
            #[cfg(debug_assertions)]
            println!("Path: {}", path.display().bright_yellow());
            if path.is_symlink() {
                if path.is_dir() {
                    #[cfg(debug_assertions)]
                    println!("Was symlink dir!");
                    if let Err(e) = symlink::remove_symlink_dir(&path) {
                        println!(
                            "Failed to remove symlink for directory {}: {}",
                            path.display().bright_yellow(),
                            e
                        );
                    }
                } else if path.is_file() {
                    #[cfg(debug_assertions)]
                    println!("Was symlink file!");
                    if let Err(e) = symlink::remove_symlink_file(&path) {
                        println!(
                            "Failed to remove symlink for file {}: {}",
                            path.display().bright_yellow(),
                            e
                        );
                    }
                } else {
                    #[cfg(debug_assertions)]
                    println!("Was broken symlink!");
                    if let Err(ed) = std::fs::remove_dir(&path) {
                        if let Err(ef) = std::fs::remove_file(&path) {
                            println!(
                            "Failed to remove broken symlink for {}:\nAttempt 1 (dir):{}\nAttempt 2 (file):{}",
                            path.display().bright_yellow(),
                            ed,
                            ef
                        );
                        }
                    }
                }
            }
        }

        remove_dir_all(&package.dependencies_dir).expect("Failed to remove cached folders");
    }
}

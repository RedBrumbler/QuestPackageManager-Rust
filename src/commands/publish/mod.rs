use clap::Args;

#[derive(Args, Debug, Clone)]

pub struct Publish {
    /// the authorization header to use for publishing, if present
    pub publish_auth: String,
}

use owo_colors::OwoColorize;

use crate::data::package::SharedPackageConfig;
pub fn execute_publish_operation(auth: &Publish) {
    let package = SharedPackageConfig::read();
    if package.config.info.url.is_none() {
        println!("Package without url can not publish!");
        return;
    }

    // check if all dependencies are available off of qpackages
    for dependency in package.config.dependencies.iter() {
        match dependency.get_shared_package() {
            Option::Some(_s) => {}
            Option::None => {
                println!(
                    "dependency {} was not available on qpackages in the given version range",
                    &dependency.id
                );
                panic!(
                    "make sure {} exists for this dependency",
                    &dependency.version_range
                );
            }
        };
    }

    // check if all required dependencies are in the restored dependencies, and if they satisfy the version ranges
    for dependency in package.config.dependencies.iter() {
        // if we can not find any dependency that matches ID and version satisfies given range, then we are missing a dep
        if let Some(el) = package
            .restored_dependencies
            .iter()
            .find(|el| el.dependency.id == dependency.id)
        {
            // if version doesn't match range, panic
            if !dependency.version_range.matches(&el.version) {
                panic!(
                    "Restored dependency {} version ({}) does not satisfy stated range ({})",
                    dependency.id.bright_red(),
                    el.version.to_string().bright_green(),
                    dependency.version_range.to_string().bright_blue()
                );
            }
        }
    }

    // check if url is set to download headers
    if package.config.info.url.is_none() {
        panic!("info.url is null, please make sure to init this with the base link to your repo, e.g. '{}'", "https://github.com/RedBrumbler/QuestPackageManager-Rust".bright_yellow());
    }
    // check if this is header only, if it's not header only check if the so_link is set, if not, panic
    if !package
        .config
        .info
        .additional_data
        .headers_only
        .unwrap_or(false)
        && package.config.info.additional_data.so_link.is_none()
    {
        panic!("soLink is not set in the package config, but this package is not header only, please make sure to either add the soLink or to make the package header only.");
    }

    // TODO: Implement a check that gets the repo and checks if the shared folder and subfolder exists, if not it throws an error and won't let you publish

    package.publish(&auth.publish_auth);

    println!(
        "Package {} v{} published!",
        package.config.info.id, package.config.info.version
    );
}


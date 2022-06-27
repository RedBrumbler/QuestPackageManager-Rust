use clap::Args;
use owo_colors::OwoColorize;

#[derive(Args, Debug, Clone)]
pub struct Package {
    pub package: String,
    #[clap(short, long)]
    pub latest: bool,
}

pub fn execute_versions_list(package: Package) {
    let versions = crate::data::qpackages::get_versions(&package.package);
    if package.latest {
        println!(
            "The latest version for package {} is {}",
            package.package.bright_red(),
            versions
                .expect("Getting version failed!")
                .get(0)
                .expect("Getting first version failed!")
                .version
                .to_string()
                .bright_green()
        );
    } else if let Some(package_versions) = &versions {
        println!(
            "Package {} has {} versions on qpackages.com:",
            package.package.bright_red(),
            versions.as_ref().unwrap().len().bright_yellow()
        );
        for package_version in package_versions.iter().rev() {
            println!(" - {}", package_version.version.to_string().bright_green());
        }
    } else {
        println!(
            "Package {} either did not exist or has no versions on qpackages.com",
            package.package.bright_red()
        );
    }
}

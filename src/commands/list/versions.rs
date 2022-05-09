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
    } else if versions.is_some() {
        println!(
            "Package {} has {} versions on qpackages.com:",
            package.package.bright_red(),
            versions.unwrap().len().bright_yellow()
        );
        for package_version in versions.unwrap().iter().rev() {
            println!(" - {}", package_version.version.to_string().bright_green());
        }
    } else {
        println!(
            "Package {} either did not exist or has no versions on qpackages.com",
            package.package.bright_red()
        );
    }
}

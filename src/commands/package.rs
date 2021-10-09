use clap::{AppSettings, Clap};

use crate::data::package::{PackageConfig, PackageInfo, AdditionalPackageData};

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Package {
    #[clap(subcommand)]
    pub op: PackageOperation
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub enum PackageOperation {
    /// Create a package
    Create(PackageOperationCreateArgs),
    /// Edit various properties of the package
    Edit(Edit),
    /// Edit extra supported properties of the package
    EditExtra
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Edit {
    ///Edit the id property of the package
    #[clap(long)]
    pub id: Option<String>,
    ///Edit the name property of the package
    #[clap(long)]
    pub name: Option<String>,
    ///Edit the url property of the package
    #[clap(long)]
    pub url: Option<String>,
    ///Edit the version property of the package
    #[clap(long)]
    pub version: Option<String>
}

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct PackageOperationCreateArgs {
    /// The name of the package
    pub name: String,
    /// The version of the package
    pub version: String,
    /// Specify an id, else lowercase will be used
    #[clap(long="id")]
    pub id: Option<String>,
    /// Branch name of a Github repo. Only used when a valid github url is provided
    #[clap(long="branchName")]
    pub branch_name: Option<String>,
    /// Specify that this package is headers only and does not contain a .so or .a file
    #[clap(long="headersOnly")]
    pub headers_only: Option<bool>,
    /// Specify that this package is static linking
    #[clap(long="staticLinking")]
    pub static_linking: Option<bool>,
    /// Specify the download link for a release .so or .a file
    #[clap(long="soLink")]
    pub so_link: Option<String>,
    /// Specify the download link for a debug .so or .a files (usually from the obj folder)
    #[clap(long="debugSoLink")]
    pub debug_so_link: Option<String>,
    /// Override the downloaded .so or .a filename with this name instead.
    #[clap(long="overrideSoName")]
    pub override_so_name: Option<String>
}

pub fn execute_package_operation(operation: Package)
{
    match operation.op {
        PackageOperation::Create(c) => package_create_operation(c),
        PackageOperation::Edit(e) => package_edit_operation(e),
        PackageOperation::EditExtra => package_edit_extra_operation()
    }
}

fn package_create_operation(create_parameters: PackageOperationCreateArgs)
{
    let additional_data = AdditionalPackageData {
        branch_name: create_parameters.branch_name,
        headers_only: create_parameters.headers_only,
        static_linking: create_parameters.static_linking,
        so_link: create_parameters.so_link,
        debug_so_link: create_parameters.debug_so_link,
        override_so_name: create_parameters.override_so_name,
        ..Default::default()
    };

    // id is optional so we need to check if it's defined, else use the name to lowercase
    let id: String;
    match create_parameters.id {
        Option::Some(s) => id = s,
        Option::None => id = create_parameters.name.to_lowercase()
    }

    let package_info = PackageInfo {
        id,
        name: create_parameters.name,
        version: create_parameters.version,
        additional_data,
        ..Default::default()
    };

    let package = PackageConfig {
        info: package_info,
        ..Default::default()
    };

    package.write();
}

fn package_edit_operation(edit_parameters: Edit)
{
    if edit_parameters.id.is_some() { package_set_id(edit_parameters.id.unwrap()); }
    if edit_parameters.name.is_some() { package_set_name(edit_parameters.name.unwrap()); }
    if edit_parameters.url.is_some() { package_set_url(edit_parameters.url.unwrap()); }
    if edit_parameters.version.is_some() { package_set_version(edit_parameters.version.unwrap()); }
}

fn package_set_id(id: String)
{
    println!("Setting package id: {}", id);
    // TODO edit mod.json and android.mk
    let mut package = PackageConfig::read();
    package.info.id = id;
    package.write();
}

fn package_set_name(name: String)
{
    println!("Setting package name: {}", name);
    let mut package = PackageConfig::read();
    package.info.name = name;
    package.write();
}

fn package_set_url(url: String)
{
    println!("Setting package url: {}", url);
    let mut package = PackageConfig::read();
    package.info.url = Option::Some(url);
    package.write();
}

fn package_set_version(version: String)
{
    println!("Setting package version: {}", version);
    // TODO  make it edit the version in mod.json and android.mk
    let mut package = PackageConfig::read();
    package.info.version = version;
    package.write();
}

fn package_edit_extra_operation()
{

}
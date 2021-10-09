use serde::{Serialize, Deserialize};
//use std::fs::{read_to_string};
use clap::{AppSettings, Clap};

mod data;
mod commands;

/// QPM is a command line tool that allows modmakers to
/// easily download dependencies for interacting with a game or other mods
#[derive(Clap, Debug)]
#[clap(version = "0.1.0", author = "RedBrumbler & Sc2ad")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// the github token to use for operations
    #[clap(short, long)]
    token: Option<String>,
    #[clap(subcommand)]
    subcmd: MainCommand
}

#[derive(Clap, Debug)]
enum MainCommand {
    /// Cache control
    Cache(commands::cache::Cache),
    /// Clear all resolved dependencies by clearing the lock file
    Clear,
    /// Collect and collapse dependencies and print them to console
    Collapse,
    /// Collect dependencies and print them to console
    Collect,
    /// Config control
    Config(commands::config::Config),
    /// Dependency control
    Dependency(commands::dependency::Dependency),
    /// Package control
    Package(commands::package::Package),
    /// List all properties that are currently supported by QPM
    PropertiesList,
    /// Publish package
    Publish,
    /// Restore and resolve all dependencies from the package
    Restore,
    /// Qmod control
    Qmod(commands::qmod::Qmod)
}

fn main() {
    let opts: Opts = Opts::parse();
    let token = opts.token;
    if token.is_some()
    {
        println!("using token {}", token.unwrap());
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd {
        MainCommand::Cache(c) => commands::cache::execute_cache_operation(c),
        MainCommand::Clear => commands::clear::execute_clear_operation(),
        MainCommand::Collapse => commands::collapse::execute_collapse_operation(),
        MainCommand::Collect => commands::collect::execute_collect_operation(),
        MainCommand::Config(c) => commands::config::execute_config_operation(c),
        MainCommand::Dependency(d) => commands::dependency::execute_dependency_operation(d),
        MainCommand::Package(p) => commands::package::execute_package_operation(p),
        MainCommand::PropertiesList => commands::propertieslist::execute_properties_list_operation(),
        MainCommand::Publish => commands::publish::execute_publish_operation(),
        MainCommand::Restore => commands::restore::execute_restore_operation(),
        MainCommand::Qmod(q) => commands::qmod::execute_qmod_operation(q)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub cache_path: String,
    pub timeout: u32
}

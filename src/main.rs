#![feature(once_cell)]
#![feature(map_try_insert)]
#![feature(let_chains)]

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

mod commands;
mod data;
mod resolver;
mod utils;

/// QPM is a command line tool that allows modmakers to
/// easily download dependencies for interacting with a game or other mods
#[derive(Parser, Debug)]
#[clap(version = "0.1.0", author = "RedBrumbler & Sc2ad")]

struct Opts {
    #[clap(subcommand)]
    subcmd: MainCommand,
}

#[derive(Subcommand, Debug, Clone)]
enum MainCommand {
    /// Cache control
    Cache(commands::cache::Cache),
    /// Clear all resolved dependencies by clearing the lock file
    Clear,
    /// Collect and collapse dependencies and print them to console
    Collapse,
    /// Config control
    Config(commands::config::Config),
    /// Dependency control
    Dependency(commands::dependency::Dependency),
    /// Package control
    Package(commands::package::Package),
    /// List all properties that are currently supported by QPM
    List(commands::list::ListOperation),
    /// Publish package
    Publish,
    /// Restore and resolve all dependencies from the package
    Restore,
    /// Qmod control
    Qmod(commands::qmod::Qmod),
    /// Install to local repository
    Install(commands::install::InstallOperation)
}

fn main() {
    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match (Opts::parse() as Opts).subcmd {
        MainCommand::Cache(c) => commands::cache::execute_cache_operation(c),
        MainCommand::Clear => commands::clear::execute_clear_operation(),
        MainCommand::Collapse => commands::collapse::execute_collapse_operation(),
        MainCommand::Config(c) => commands::config::execute_config_operation(c),
        MainCommand::Dependency(d) => commands::dependency::execute_dependency_operation(d),
        MainCommand::Package(p) => commands::package::execute_package_operation(p),
        MainCommand::List(l) => commands::list::execute_list_operation(l),
        MainCommand::Publish => commands::publish::execute_publish_operation(),
        MainCommand::Restore => commands::restore::execute_restore_operation(),
        MainCommand::Qmod(q) => commands::qmod::execute_qmod_operation(q),
        MainCommand::Install(i) => commands::install::execute_install_operation(i),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub cache_path: String,
    pub timeout: u32,
}
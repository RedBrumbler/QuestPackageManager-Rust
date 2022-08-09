use clap::{Subcommand, Args};

mod cache;
mod ndkpath;
mod symlink;
mod timeout;
mod token;
mod publish;

use owo_colors::OwoColorize;

use crate::data::config::Config as AppConfig;

#[derive(Args, Debug, Clone)]

pub struct Config {
    /// The operation to execute
    #[clap(subcommand)]
    pub op: ConfigOperation,
    /// use this flag to edit the local config instead of the global one
    #[clap(short, long)]
    pub local: bool,
}

#[derive(Subcommand, Debug, Clone)]

pub enum ConfigOperation {
    /// Get or set the cache path
    Cache(cache::Cache),
    /// Enable or disable symlink usage
    Symlink(symlink::Symlink),
    /// Get or set the timeout for web requests
    Timeout(timeout::Timeout),
    /// Get or set the github token used for restore
    Token(token::Token),
    /// Print the location of the global config
    Location,
    /// Get or set the ndk path used in generation of build files
    NDKPath(ndkpath::NDKPath),
    /// Get or set the publish key used for publish
    Publish(publish::Key),
}

pub fn execute_config_operation(operation: Config) {
    let mut config = if operation.local {
        AppConfig::read_local()
    } else {
        AppConfig::read()
    };

    let mut changed_any = false;
    match operation.op {
        ConfigOperation::Cache(c) => {
            changed_any = cache::execute_cache_config_operation(&mut config, c)
        }
        ConfigOperation::Symlink(s) => {
            changed_any = symlink::execute_symlink_config_operation(&mut config, s)
        }
        ConfigOperation::Timeout(t) => {
            changed_any = timeout::execute_timeout_config_operation(&mut config, t)
        }
        ConfigOperation::Token(t) => token::execute_token_config_operation(t),
        ConfigOperation::Location => println!(
            "Global Config is located at {}",
            AppConfig::global_config_path().display().bright_yellow()
        ),
        ConfigOperation::NDKPath(p) => {
            changed_any = ndkpath::execute_ndk_config_operation(&mut config, p)
        },
        ConfigOperation::Publish(k) => publish::execute_key_config_operation(k),
    }

    if !changed_any {
        return;
    }

    if operation.local {
        config.write_local();
    } else {
        config.write();
    }
}

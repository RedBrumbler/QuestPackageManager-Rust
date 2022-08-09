use clap::Args;
use owo_colors::OwoColorize;

use crate::data::config::get_publish_keyring;

#[derive(Args, Debug, Clone)]
pub struct Key {
    pub key: Option<String>,
    #[clap(long)]
    pub delete: bool,
}

pub fn execute_key_config_operation(operation: Key) {
    if operation.delete && get_publish_keyring().get_password().is_ok() {
        get_publish_keyring()
            .delete_password()
            .expect("Removing publish key failed");
        println!("Deleted publish key from config, it will no longer be used");
        return;
    } else if operation.delete {
        println!("There was no publish key configured, did not delete it");
        return;
    }

    if let Some(key) = operation.key {
        // write key
        get_publish_keyring().set_password(&key).expect("Failed to set publish key");
        println!("Configured a publish key! This will now be used for future qpm publish calls");
    } else {
        // read token, possibly unused so prepend with _ to prevent warnings
        if let Ok(_key) = get_publish_keyring().get_password() {
            #[cfg(debug_assertions)]
            println!("Configured publish key: {}", _key.bright_yellow());
            #[cfg(not(debug_assertions))]
            println!(
                "In release builds you {} view the configured publish key!",
                "cannot".bright_red()
            );
        } else {
            println!("No publish key was configured, or getting the publish key failed!");
        }
    }
}

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symlink: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ndk_path: Option<String>
}

impl Default for Config {
    #[inline]
    fn default() -> Config {
        Config {
            symlink: Some(true),
            cache: Some(dirs::data_dir().unwrap().join("QPM-Rust").join("cache")),
            timeout: Some(5000),
            ndk_path: None,
        }
    }
}

impl Config {
    /// always gets the global config
    pub fn read() -> Config {
        let path = Config::global_config_path();
        std::fs::create_dir_all(Config::global_config_dir()).expect("Failed to make config folder");

        if let Ok(file) = std::fs::File::open(path) {
            // existed
            serde_json::from_reader(file).expect("Deserializing global config failed")
        } else {
            // didn't exist
            Config {
                ..Default::default()
            }
        }
    }

    pub fn read_local() -> Config {
        let path = "qpm.settings.json";
        if let Ok(file) = std::fs::File::open(path) {
            // existed
            serde_json::from_reader(file).expect(&format!("Deserializing {} failed", path))
        } else {
            // didn't exist
            Config {
                symlink: None,
                cache: None,
                timeout: None,
                ndk_path: None,
            }
        }
    }

    /// combines the values of the global config with whatever is written in a local qpm.settings.json
    pub fn read_combine() -> Config {
        let mut config = Config::read();

        // read a local qpm.settings.json to
        let local_path = "qpm.settings.json";
        if let Ok(file) = std::fs::File::open(local_path) {


            let local_config: Config =
                serde_json::from_reader(file).expect("Deserializing package failed");

            if local_config.symlink.is_some() {
                config.symlink = local_config.symlink;
            }
            if local_config.cache.is_some() {
                config.cache = local_config.cache;
            }
            if local_config.timeout.is_some() {
                config.timeout = local_config.timeout;
            }
            if local_config.ndk_path.is_some() {
                config.ndk_path = local_config.ndk_path;
            }
        }

        config
    }

    pub fn write(&self) {
        let path = Config::global_config_path();

        std::fs::create_dir_all(Config::global_config_dir()).expect("Failed to make config folder");
        let file = std::fs::File::create(path).expect("create failed");
        serde_json::to_writer_pretty(file, &self).expect("Serialization failed");

        println!("Saved Config!");
    }

    pub fn write_local(&self) {
        std::fs::create_dir_all(Config::global_config_dir()).expect("Failed to make config folder");
        let path = "qpm.settings.json";
        let file = std::fs::File::create(path).expect("create failed");

        serde_json::to_writer_pretty(file, &self).expect("Serialization failed");
        println!("Saved Config!");
    }

    pub fn global_config_path() -> PathBuf {
        Config::global_config_dir().join("qpm.settings.json")
    }

    pub fn global_config_dir() -> PathBuf {
        dirs::config_dir().unwrap().join("QPM-Rust")
    }
}

#[inline]
pub fn get_keyring() -> keyring::Entry {
    keyring::Entry::new("qpm", "github")
}
#[inline]
pub fn get_publish_keyring() -> keyring::Entry {
    keyring::Entry::new("qpm", "publish")
}

use std::{collections::HashMap, io::Cursor, lazy::SyncLazy as Lazy, time::Duration};

use atomic_refcell::AtomicRefCell;
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::data::{config::Config, package::SharedPackageConfig};
static API_URL: &str = "https://qpackages.com";
static AUTH_HEADER: &str = "not that i can come up with";

static VERSIONS_CACHE: Lazy<AtomicRefCell<HashMap<String, Vec<PackageVersion>>>> =
    Lazy::new(Default::default);
static SHARED_PACKAGE_CACHE: Lazy<AtomicRefCell<HashMap<String, SharedPackageConfig>>> =
    Lazy::new(Default::default);

static AGENT: Lazy<AtomicRefCell<ureq::Agent>> = Lazy::new({
    || {
        AtomicRefCell::new(
            ureq::AgentBuilder::new()
                .timeout_read(Duration::from_millis(
                    Config::read_combine().timeout.unwrap(),
                ))
                .timeout_write(Duration::from_millis(
                    Config::read_combine().timeout.unwrap(),
                ))
                .user_agent(
                    format!("questpackagemanager-rust/{}", env!("CARGO_PKG_VERSION")).as_str(),
                )
                .build(),
        )
    }
});

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
pub struct  PackageVersion {
    pub id: String,
    pub version: Version,
}

/// Requests the appriopriate package info from qpackage.com
pub fn get_versions(id: &str) -> Vec<PackageVersion> {
    let url = format!("{}/{}?limit=0", API_URL, id);

    if let Some(entry) = VERSIONS_CACHE.borrow().get(&url) {
        return entry.clone();
    }

    let versions = AGENT
        .borrow_mut()
        .get(&url)
        .call()
        .expect("Request to qpackages.com failed")
        .into_json::<Vec<PackageVersion>>()
        .expect("Into json failed");

    VERSIONS_CACHE.borrow_mut().insert(url, versions.clone());
    versions
}

pub fn get_shared_package(id: &str, ver: &Version) -> SharedPackageConfig {
    let url = format!("{}/{}/{}", API_URL, id, ver);

    if let Some(entry) = SHARED_PACKAGE_CACHE.borrow().get(&url) {
        return entry.clone();
    }

    let shared_package = AGENT
        .borrow_mut()
        .get(&url)
        .call()
        .expect("Request to qpackages.com failed")
        .into_json::<SharedPackageConfig>()
        .expect("Into json failed");

    SHARED_PACKAGE_CACHE
        .borrow_mut()
        .insert(url, shared_package.clone());
    shared_package
}

pub fn get_packages() -> Vec<String> {
    AGENT
        .borrow_mut()
        .get(API_URL)
        .call()
        .expect("Request to qpackages.com failed")
        .into_json::<Vec<String>>()
        .expect("Into json failed")
}

pub fn publish_package(package: &SharedPackageConfig) {
    let url = format!(
        "{}/{}/{}",
        API_URL, &package.config.info.id, &package.config.info.version
    );

    let s = serde_json::to_string_pretty(&package).expect("json failed");
    let read = Cursor::new(s.into_bytes());
    AGENT
        .borrow_mut()
        .post(&url)
        .set("Authorization", AUTH_HEADER)
        .send(read)
        .expect("Request to qpackages.com failed");
}

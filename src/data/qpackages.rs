use std::{collections::HashMap, sync::LazyLock as Lazy};

use atomic_refcell::AtomicRefCell;
use reqwest::blocking::Response;
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::{data::{package::SharedPackageConfig}, utils::network::get_agent};
static API_URL: &str = "https://qpackages.com";

static VERSIONS_CACHE: Lazy<AtomicRefCell<HashMap<String, Vec<PackageVersion>>>> =
    Lazy::new(Default::default);
static SHARED_PACKAGE_CACHE: Lazy<AtomicRefCell<HashMap<String, SharedPackageConfig>>> =
    Lazy::new(Default::default);



#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
pub struct PackageVersion {
    pub id: String,
    pub version: Version,
}

// true if 404
fn is_404_or_panic(res: &Result<Response, reqwest::Error>) -> bool {
    if let Err(e) = res {

        if let Some(status) = e.status() {
            if status == 404u16 {
                return true;
            }

            panic!("Received error code {:?} with response {:?}", status, &res)
        }

        panic!("Unable to send request {}", dbg!(&e));
    }

    false
}

/// Requests the appriopriate package info from qpackage.com
pub fn get_versions(id: &str) -> Option<Vec<PackageVersion>> {
    let url = format!("{}/{}?limit=0", API_URL, id);

    if let Some(entry) = VERSIONS_CACHE.borrow().get(&url) {
        return Some(entry.clone());
    }

    let response = get_agent()
        .get(&url)
        .send();

    if is_404_or_panic(&response) {
        return None;
    }

    let versions: Vec<PackageVersion> = response
        .expect("Request to qpackages.com failed")
        .json()
        .expect("Into json failed");
        

    VERSIONS_CACHE.borrow_mut().insert(url, versions.clone());


    Some(versions)
}

pub fn get_shared_package(id: &str, ver: &Version) -> Option<SharedPackageConfig> {
    let url = format!("{}/{}/{}", API_URL, id, ver);

    if let Some(entry) = SHARED_PACKAGE_CACHE.borrow().get(&url) {
        return Some(entry.clone());
    }

    let response = get_agent()
        .get(&url)
        .send();

    let shared_package: SharedPackageConfig = response
        .expect("Request to qpackages.com failed")
        .json()
        .expect("Into json failed");

    SHARED_PACKAGE_CACHE
        .borrow_mut()
        .insert(url, shared_package.clone());
    Some(shared_package)
}

pub fn get_packages() -> Vec<String> {
    get_agent()
        .get(API_URL)
        .send()
        .expect("Request to qpackages.com failed")
        .json()
        .expect("Into json failed")
}

pub fn publish_package(package: &SharedPackageConfig, auth: &str) {
    let url = format!(
        "{}/{}/{}",
        API_URL, &package.config.info.id, &package.config.info.version
    );

    get_agent()
        .post(&url)
        .header("Authorization", auth)
        .json(&package)
        .send()
        .expect("Request to qpackages.com failed");
}

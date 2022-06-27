use semver::VersionReq;
use serde::{Deserialize, Serialize};

use super::AdditionalDependencyData;
use crate::data::{package::SharedPackageConfig, qpackages};

#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    pub id: String,
    #[serde(deserialize_with = "cursed_semver_parser::deserialize")]
    pub version_range: VersionReq,
    pub additional_data: AdditionalDependencyData,
}

/*
#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalDependencyData {
    /// Copy a dependency from a location that is local to this root path instead of from a remote url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_path: Option<String>,

    /// Whether or not the package is header only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers_only: Option<bool>,

    /// Whether or not the package is statically linked
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_linking: Option<bool>,

    /// Whether to use the release or debug .so for linking
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_release: Option<bool>,

    /// the link to the so file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub so_link: Option<String>,

    /// the link to the debug .so file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_so_link: Option<String>,

    /// the overridden so file name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_so_name: Option<String>,

    /// the link to the qmod
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mod_link: Option<String>,

    /// Branch name of a Github repo. Only used when a valid github url is provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,

    /// Specify any additional files to be downloaded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_files: Option<Vec<String>>,

    /// Whether or not the dependency is private and should be used in restore
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename(serialize = "private", deserialize = "private")
    )]
    pub is_private: Option<bool>,
}

*/
/*
impl Default for AdditionalDependencyData {
    fn default() -> Self {
        Self {
            local_path: None,
            headers_only: None,
            static_linking: None,
            use_release: None,
            so_link: None,
            debug_so_link: None,
            override_so_name: None,
            mod_link: None,
            branch_name: None,
            extra_files: None,
            is_private: None,
        }
    }
}
*/
/*
impl AdditionalDependencyData {
    pub fn merge(&mut self, other: AdditionalDependencyData) {
        if self.branch_name.is_none() {
            if let Some(other_branch_name) = &other.branch_name {
                self.branch_name = Some(other_branch_name.clone());
            }
        }

        if let (Some(extra_files), Some(other_extra_files)) =
            (&mut self.extra_files, &other.extra_files)
        {
            extra_files.append(&mut other_extra_files.clone());
        } else if self.extra_files.is_none() {
            if let Some(other_extra_files) = &other.extra_files {
                self.extra_files = Some(other_extra_files.clone());
            }
        }

        if self.local_path.is_none() {
            if let Some(other_local_path) = &other.local_path {
                self.local_path = Some(other_local_path.clone());
            }
        }

        if let (Some(is_private), Some(other_is_private)) = (&self.is_private, &other.is_private) {
            self.is_private = Some(*is_private || *other_is_private);
        } else if self.is_private.is_none() {
            if let Some(other_is_private) = &other.is_private {
                self.is_private = Some(*other_is_private);
            }
        }
    }

    pub fn merge_package(&mut self, other: AdditionalPackageData) {
        if let Some(static_linking) = other.static_linking {
            self.static_linking = Some(static_linking);
        }

        if self.mod_link.is_none() {
            self.mod_link = other.mod_link;
        }
    }
}
*/

impl Dependency {
    pub fn get_shared_package(&self) -> Option<SharedPackageConfig> {
        let versions = qpackages::get_versions(&self.id).expect("Unable to get versions");
        for v in versions.iter() {
            if self.version_range.matches(&v.version) {
                return qpackages::get_shared_package(&self.id, &v.version);
            }
        }

        Option::None
    }
}

/*
impl From<AdditionalPackageData> for AdditionalDependencyData {
    fn from(package_data: AdditionalPackageData) -> Self {
        serde_json::from_str(&serde_json::to_string(&package_data).unwrap()).unwrap()
    }
}
*/

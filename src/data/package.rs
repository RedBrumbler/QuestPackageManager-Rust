use std::path::PathBuf;

use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

use crate::data::{
    dependency::{AdditionalDependencyData, Dependency},
    shared_package::SharedPackageConfig,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompileOptions {
    /// Additional include paths to add, relative to the extern directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_paths: Option<Vec<String>>,

    /// Additional system include paths to add, relative to the extern directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_includes: Option<Vec<String>>,

    /// Additional C++ features to add.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpp_features: Option<Vec<String>>,

    /// Additional C++ flags to add.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpp_flags: Option<Vec<String>>,

    /// Additional C flags to add.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_flags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPackageData {
    /// Whether or not the package is header only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers_only: Option<bool>,

    /// Whether or not the package is statically linked
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_linking: Option<bool>,

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

    /// Whether to use the release .so for linking
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_release: Option<bool>,

    /// Additional Compile options to be used with this package
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compile_options: Option<CompileOptions>,

    /// Sub folder to use from the downloaded repo / zip, so one repo can contain multiple packages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_folder: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageInfo {
    pub name: String,
    pub id: String,
    pub version: Version,
    pub url: Option<String>,
    pub additional_data: AdditionalPackageData,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageConfig {
    pub shared_dir: PathBuf,
    pub dependencies_dir: PathBuf,
    pub info: PackageInfo,
    pub dependencies: Vec<Dependency>,
    pub additional_data: AdditionalDependencyData,
}

impl PackageConfig {
    pub fn write(&self) {
        let file = std::fs::File::create("qpm.json").expect("create failed");
        serde_json::to_writer_pretty(file, &self).expect("Serialization failed");
        println!("Package {} Written!", self.info.id);
    }

    pub fn check() -> bool {
        std::path::Path::new("qpm.json").exists()
    }

    pub fn read() -> PackageConfig {
        let file = std::fs::File::open("qpm.json").expect("Opening qpm.json failed");
        serde_json::from_reader(file).expect("Deserializing package failed")
    }

    pub fn add_dependency(&mut self, dependency: Dependency) {
        let dep = self.get_dependency(&dependency.id);
        match dep {
            Option::Some(_d) => {
                println!(
                    "Not adding dependency {} because it already existed",
                    &dependency.id
                );
            }
            Option::None => {
                self.dependencies.push(dependency);
            }
        }
    }

    pub fn get_dependency(&mut self, id: &str) -> Option<&mut Dependency> {
        for (idx, dependency) in self.dependencies.iter().enumerate() {
            if dependency.id.eq(id) {
                return self.dependencies.get_mut(idx);
            }
        }

        Option::default()
    }

    pub fn remove_dependency(&mut self, id: &str) {
        for (idx, dependency) in self.dependencies.iter().enumerate() {
            if dependency.id.eq(id) {
                println!("removed dependency {}", id);
                self.dependencies.remove(idx);
                return;
            }
        }

        println!("Not removing dependency {} because it did not exist", id);
    }

    pub fn resolve(&self) -> impl Iterator<Item = SharedPackageConfig> + '_ {
        crate::resolver::resolve(self)
    }

    pub fn get_module_id(&self) -> String {
        let name = self.get_so_name();
        if self.additional_data.static_linking.unwrap_or(false) {
            name[3..name.len() - 2].to_string()
        } else {
            name[3..name.len() - 3].to_string()
        }
    }

    pub fn get_so_name(&self) -> String {
        self.info
            .additional_data
            .override_so_name
            .clone()
            .unwrap_or(format!(
                "lib{}_{}.{}",
                self.info.id,
                self.info.version.to_string().replace('.', "_"),
                if self.additional_data.static_linking.unwrap_or(false) {
                    "a"
                } else {
                    "so"
                },
            ))
    }
}

fn intersect(mut lhs: VersionReq, mut rhs: VersionReq) -> VersionReq {
    lhs.comparators.append(&mut rhs.comparators);
    lhs
}

impl From<AdditionalDependencyData> for AdditionalPackageData {
    fn from(dependency_data: AdditionalDependencyData) -> Self {
        serde_json::from_str(&serde_json::to_string(&dependency_data).unwrap()).unwrap()
    }
}

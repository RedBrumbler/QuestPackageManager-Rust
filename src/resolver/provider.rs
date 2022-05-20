use std::borrow::Borrow;

use itertools::Itertools;
use pubgrub::{range::Range, solver::Dependencies};

use super::semver::{req_to_range, Version};
use crate::data::{
    package::{PackageConfig, SharedPackageConfig},
    qpackages::{self, PackageVersion},
};

pub trait DependencyRepository {
    fn get_versions(&self, id: &str) -> Option<Vec<PackageVersion>>;
    fn get_shared_package(
        &self,
        id: &str,
        version: &semver::Version,
    ) -> Option<SharedPackageConfig>;
}

pub struct DependencyProvider<'a> {
    root: &'a PackageConfig,
    repositories: Vec<Box<dyn DependencyRepository>>,
}

impl<'a> DependencyProvider<'a> {
    // Repositories sorted in order
    pub fn new(root: &'a PackageConfig, repositories: Vec<Box<dyn DependencyRepository>>) -> Self {
        Self { root, repositories }
    }
}

/// 
/// Merge multiple repositories into one
/// Allow fetching from multiple backends
/// 
impl DependencyRepository for DependencyProvider<'_> {
    // get versions of all repositories
    fn get_versions(&self, id: &str) -> Option<Vec<PackageVersion>> {
        // double flat map???? rust weird
        let mut result: Vec<PackageVersion> = self
            .repositories
            .iter()
            .flat_map(|r| r.get_versions(id))
            .flatten()
            .unique()
            .collect();

        // we add ourselves to the gotten versions, so the local version always can be resolved as most ideal
        if *id == self.root.info.id {
            result.push(qpackages::PackageVersion {
                id: self.root.info.id.clone(),
                version: self.root.info.version.clone(),
            });
        }

        if result.is_empty() {
            return None;
        }


        Some(result)
    }

    // get package from the first repository that has it
    fn get_shared_package(
        &self,
        id: &str,
        version: &semver::Version,
    ) -> Option<SharedPackageConfig> {
        self.repositories
            .iter()
            .find_map(|r| r.get_shared_package(id, version))
    }
}

impl pubgrub::solver::DependencyProvider<String, Version> for DependencyProvider<'_> {
    fn choose_package_version<T: Borrow<String>, U: Borrow<Range<Version>>>(
        &self,
        potential_packages: impl Iterator<Item = (T, U)>,
    ) -> Result<(T, Option<Version>), Box<dyn std::error::Error>> {
        Ok(pubgrub::solver::choose_package_with_fewest_versions(
            |id| {
                self.get_versions(id)
                    // TODO: Anyhow
                    .unwrap_or_else(|| panic!("Unable to find versions for package {id}"))
                    .into_iter()
                    .map(|pv: qpackages::PackageVersion| pv.version.into())
            },
            potential_packages,
        ))
    }

    fn get_dependencies(
        &self,
        id: &String,
        version: &Version,
    ) -> Result<Dependencies<String, Version>, Box<dyn std::error::Error>> {
        if id == &self.root.info.id && version == &self.root.info.version {
            let deps = self
                .root
                .dependencies
                .iter()
                .map(|dep| {
                    let id = &dep.id;
                    let version = req_to_range(dep.version_range.clone());
                    (id.clone(), version)
                })
                .collect();
            Ok(Dependencies::Known(deps))
        } else {
            let mut package = self
                .get_shared_package(id, &version.clone().into())
                .unwrap_or_else(|| panic!("Could not find package {id} with version {version}"));
            // remove any private dependencies
            package
                .config
                .dependencies
                .retain(|dep| !dep.additional_data.is_private.unwrap_or(false));

            let deps = package
                .config
                .dependencies
                .into_iter()
                .map(|dep| {
                    let id = dep.id;
                    let version = req_to_range(dep.version_range);
                    (id, version)
                })
                .collect();
            Ok(Dependencies::Known(deps))
        }
    }
}

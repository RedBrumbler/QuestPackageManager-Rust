use itertools::Itertools;

use crate::data::{package::{SharedPackageConfig}, file_repository::FileRepository, qpackages::PackageVersion};

use super::{DependencyRepository, qpm_provider::QPMRepository};


pub fn default_repositories() -> Vec<Box<dyn DependencyRepository>> {
    // TODO: Make file repository cached
    let file_repository = Box::new(FileRepository::read());
    let qpm_repository = Box::new(QPMRepository::new());
    vec![file_repository, qpm_repository]
}

pub struct MultiDependencyProvider {
    repositories: Vec<Box<dyn DependencyRepository>>,
}

impl<'a> MultiDependencyProvider {
    // Repositories sorted in order
    pub fn new( repositories: Vec<Box<dyn DependencyRepository>>) -> Self {
        Self { repositories }
    }

    pub fn useful_default_new() -> Self {
        MultiDependencyProvider::new(default_repositories())
    }
}

/// 
/// Merge multiple repositories into one
/// Allow fetching from multiple backends
/// 
impl DependencyRepository for MultiDependencyProvider {
    // get versions of all repositories
    fn get_versions(&self, id: &str) -> Option<Vec<PackageVersion>> {
        // double flat map???? rust weird
        let result: Vec<PackageVersion> = self
            .repositories
            .iter()
            .flat_map(|r| r.get_versions(id))
            .flatten()
            .unique()
            .collect();

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
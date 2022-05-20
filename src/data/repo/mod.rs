use super::{package::SharedPackageConfig, qpackages::{PackageVersion}, dependency::SharedDependency};


pub mod local_provider;
pub mod qpm_provider;
pub mod multi_provider;

pub trait DependencyRepository {
    fn get_versions(&self, id: &str) -> Option<Vec<PackageVersion>>;
    fn get_shared_package(
        &self,
        id: &str,
        version: &semver::Version,
    ) -> Option<SharedPackageConfig>;

    fn get_shared_package_from_dependency(&self, shared_package: &SharedDependency) -> Option<SharedPackageConfig> where Self: Sized {
        self.get_shared_package(&shared_package.dependency.id, &shared_package.version)
    }
}


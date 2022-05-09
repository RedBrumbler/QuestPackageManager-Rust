use semver::Version;

use crate::data::qpackages;

use super::{provider::DependencyRepository};

pub struct QPMRepository {}

impl QPMRepository {
    pub fn new() -> Self {
        QPMRepository {  }
    }
}

impl DependencyRepository for QPMRepository {
    fn get_versions(&self, id: &String) -> Option<Vec<crate::data::qpackages::PackageVersion>> {
        qpackages::get_versions(id)
    }

    fn get_shared_package(&self, id: &String, version: &Version) -> Option<crate::data::package::SharedPackageConfig> {
        qpackages::get_shared_package(id, version)
    }
}
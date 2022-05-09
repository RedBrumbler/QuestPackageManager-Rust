use semver::Version;

use super::provider::DependencyRepository;
use crate::data::{file_repository::FileRepository, qpackages::PackageVersion};

impl DependencyRepository for FileRepository {
    fn get_versions(&self, id: &String) -> Option<Vec<crate::data::qpackages::PackageVersion>> {
        match self.get_artifacts_from_id(id) {
            Some(artifacts) => Some(
                artifacts
                    .keys()
                    .map(|version| PackageVersion {
                        id: id.clone(),
                        version: version.clone(),
                    })
                    .collect(),
            ),
            None => None,
        }
    }

    fn get_shared_package(
        &self,
        id: &String,
        version: &Version,
    ) -> Option<crate::data::package::SharedPackageConfig> {
        self.get_artifact(id, version).cloned()
    }
}

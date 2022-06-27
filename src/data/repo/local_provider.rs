use semver::Version;


use crate::data::{file_repository::FileRepository, qpackages::PackageVersion};

use super::DependencyRepository;

impl DependencyRepository for FileRepository {
    fn get_versions(&self, id: &str) -> Option<Vec<crate::data::qpackages::PackageVersion>> {
        self.get_artifacts_from_id(id).map(|artifacts| {
            artifacts
                .keys()
                .map(|version| PackageVersion {
                    id: id.to_string(),
                    version: version.clone(),
                })
                .collect()
        })
    }

    fn get_shared_package(
        &self,
        id: &str,
        version: &Version,
    ) -> Option<crate::data::package::SharedPackageConfig> {
        self.get_artifact(id, version).cloned()
    }
}

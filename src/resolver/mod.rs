use std::process;

use pubgrub::{
    error::PubGrubError,
    report::{DefaultStringReporter, Reporter},
};

use self::{provider::{DependencyProvider, DependencyRepository}, qpm_provider::QPMRepository};
use crate::data::{
    package::{PackageConfig, SharedPackageConfig},
    file_repository::{FileRepository},
};

mod provider;
mod semver;
mod local_provider;
mod qpm_provider;

fn repositories() -> Vec<Box<dyn DependencyRepository>> {
    // TODO: Make file repository cached
    let file_repository = Box::new(FileRepository::read());
    let qpm_repository = Box::new(QPMRepository::new());
    vec![file_repository, qpm_repository]
}

pub fn resolve(root: &PackageConfig) -> impl Iterator<Item = SharedPackageConfig> + '_ {
    let provider = DependencyProvider::new(root, repositories());
    match pubgrub::solver::resolve(&provider, root.info.id.clone(), root.info.version.clone()) {
        Ok(deps) => deps
            .into_iter()
            .filter_map(move |(id, version)| {
                if !(id == root.info.id && version == root.info.version) {
                    return None;
                }
        
                provider.get_shared_package(&id, &version.into())
            }),

        Err(PubGrubError::NoSolution(tree)) => {
            let report = DefaultStringReporter::report(&tree);
            eprintln!("failed to resolve dependencies: \n{}", report);
            process::exit(1)
        }
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1)
        }
    }
}

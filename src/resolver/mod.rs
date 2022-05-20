use std::process;

use pubgrub::{
    error::PubGrubError,
    report::{DefaultStringReporter, Reporter},
};


use crate::data::{
    package::{PackageConfig, SharedPackageConfig}, repo::{DependencyRepository, multi_provider::MultiDependencyProvider},
};

use self::provider::HackDependencyProvider;

mod provider;
mod semver;



pub fn resolve(root: &PackageConfig) -> impl Iterator<Item = SharedPackageConfig> + '_ {
    let provider = HackDependencyProvider::new(root, MultiDependencyProvider::useful_default_new());
    match pubgrub::solver::resolve(&provider, root.info.id.clone(), root.info.version.clone()) {
        Ok(deps) => deps
            .into_iter()
            .filter_map(move |(id, version)| {
                if id == root.info.id && version == root.info.version {
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

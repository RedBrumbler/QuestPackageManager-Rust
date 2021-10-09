use serde::{Deserialize, Serialize};
use crate::data::package;
use crate::data::shared_dependency::SharedDependency;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SharedPackageConfig {
    pub config: package::PackageConfig,
    pub restored_dependencies: Vec<SharedDependency>
}

#[allow(dead_code)]
impl SharedPackageConfig {
    pub fn read() -> SharedPackageConfig
    {
        let file = std::fs::File::open("qpm.shared.json").expect("Opening qpm.json failed");
        // Open the file in read-only mode with buffer.
        let reader = BufReader::new(file);

        serde_json::from_reader::<_, SharedPackageConfig>(reader).expect("Deserializing package failed")
    }

    pub fn write(&self)
    {
        let file = std::fs::File::create("qpm.shared.json").expect("create failed");
        serde_json::to_writer_pretty::<_, SharedPackageConfig>(file, self).expect("Unable to write qpm.shared.json");
        println!("Package {} Written!", self.config.info.id);
    }

    pub fn collect(&mut self) -> Vec<SharedDependency>
    {
        let mut deps =  Vec::<SharedDependency>::new();
        deps.append(&mut self.restored_dependencies);
        for dependency in &self.restored_dependencies
        {
            let mut their_shared = dependency.get_shared_package();
            deps.append(&mut their_shared.collect());
        }

        deps
    }

    pub fn publish(&self)
    {
        for dependency in self.config.dependencies.iter()
        {
            match dependency.get_shared_package() {
                Option::Some(_s) => {},
                Option::None => {
                    println!("dependency {} was not available on qpackages in the given version range", &dependency.id);
                    println!("make sure {} exists for this dependency", &dependency.version_range);
                    std::process::exit(0);
                }
            };
        }
    }
}
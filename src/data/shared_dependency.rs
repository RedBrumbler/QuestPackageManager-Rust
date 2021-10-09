use serde::{Serialize, Deserialize};
use crate::data::dependency::Dependency;
use crate::data::shared_package::SharedPackageConfig;
use crate::data::qpackages;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::cmp::Eq;
use semver::{Version};
use std::collections::hash_map::DefaultHasher;

#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SharedDependency {
    pub dependency: Dependency, // TODO: Instead of cloning, reference?
    pub version: String
}

#[allow(dead_code)]
impl SharedDependency {
    pub fn get_shared_package(&self) -> SharedPackageConfig
    {
        qpackages::get_shared_package(&self.dependency.id, &self.version)
    }

    pub fn collect(&self, this_id: &str, collected: &mut HashMap<SharedDependency, SharedPackageConfig>)
    {
        if this_id.eq(&self.dependency.id)
        {
            return;
        }

        if !collected.contains_key(self)
        {
            let mut found = false;

            // some need to be removed and imo this is the easiest way to do it because borrowing is a bitch and I hate it
            collected.retain(|shared_dependency, _| {
                // if the passed thing is the same ID as self
                if shared_dependency.dependency.id.eq(&self.dependency.id)
                {
                    // if it has an override so name
                    if let Some(override_so_name) = shared_dependency.get_shared_package().config.info.additional_data.override_so_name {
                        // if self has an override so name
                        if let Some(self_override_so_name) = self.get_shared_package().config.info.additional_data.override_so_name {
                            // if they are the same
                            if override_so_name.to_lowercase().eq(&self_override_so_name.to_lowercase())
                            {
                                // if self version is higher, remove it (and we add it back later)
                                if Version::parse(&self.version).unwrap() > Version::parse(&shared_dependency.version).unwrap()
                                {
                                    return false;
                                }
                                else
                                {
                                    // we found a good dep, we don't need to add this one again
                                    found = true;
                                }
                            }
                        }
                    }
                    // if id == id (checked before) & version is exact same, don't add it again
                    else if self.version.eq(&shared_dependency.version)
                    {
                        found = true;
                    }
                }
                
                true
            });

            // if we didn't find a valid version
            if !found
            {
                // get shared package to add to the hashmap
                let mut shared_package = self.get_shared_package();


                
                // remove private deps
                shared_package.restored_dependencies.retain(|restored_dependency| {
                    if let Some(is_private) = &restored_dependency.dependency.additional_data.is_private {
                        return !is_private;
                    }
        
                    true
                });
                
                // insert into hashmap
                collected.insert(self.clone(), shared_package.clone());
                
                // collect all the shared deps
                for shared_dependency in shared_package.restored_dependencies.iter()
                {
                    shared_dependency.collect(&shared_package.config.info.id, collected);
                }
            }
        }
    }

    pub fn get_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}
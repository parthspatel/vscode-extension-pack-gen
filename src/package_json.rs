use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use crate::pack_config::{Module, PackConfig};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    pub name: String,
    pub display_name: String,
    pub version: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub author: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub publisher: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub license: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub repository: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub icon: String,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub engines: HashMap<String, String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub extension_pack: Vec<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub scripts: HashMap<String, String>,
    #[serde(skip_serializing)]
    pub overwrite: bool,
}

impl From<PackConfig> for Vec<PackageJson> {
    fn from(pack: PackConfig) -> Self {
        pack.modules.into_iter().map(|module: Module|
            PackageJson {
                name: format!("{}{}", pack.common.name_prefix.clone(), module.name),
                display_name: format!("{}{}", pack.common.display_name_prefix.clone(), module.display_name),
                version: pack.common.version.clone(),
                description: module.description,
                author: pack.common.author.clone(),
                publisher: pack.common.publisher.clone(),
                license: pack.common.license.clone(),
                repository: pack.common.repository.clone(),
                icon: "".to_string(),
                engines: pack.common.engines.clone(),
                categories: merge_and_deduplicate(vec![
                    pack.common.categories.clone(),
                    module.categories,
                ]),
                extension_pack: module.packages,
                scripts: default_build_script(),
                overwrite: pack.common.overwrite.clone(),
            }
        ).collect()
    }
}

fn default_build_script() -> HashMap<String, String> {
    HashMap::from([("build-extension".to_string(), "vsce package".to_string())])
}

fn merge_and_deduplicate<T: Eq + std::hash::Hash + Clone>(vectors: Vec<Vec<T>>) -> Vec<T> {
    let mut set = HashSet::new();
    let mut result = Vec::new();

    for vec in vectors {
        for item in vec {
            if set.insert(item.clone()) {
                result.push(item);
            }
        }
    }

    result
}

fn deduplicate<T: Eq + std::hash::Hash + Clone>(vec: Vec<T>) -> Vec<T> {
    let mut set = HashSet::new();
    let mut result = Vec::new();

    for item in vec {
        if set.insert(item.clone()) {
            result.push(item);
        }
    }

    result
}

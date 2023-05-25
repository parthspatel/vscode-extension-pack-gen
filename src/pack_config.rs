use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use image::{GenericImageView, ImageFormat, Pixel, RgbaImage};
use serde::{Deserialize, Deserializer, Serialize};

fn default_engine() -> HashMap<String, String> { HashMap::from([("vscode".to_string(), " ^ 1.78.0".to_string())]) }

fn default_bool_false() -> bool { false }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackConfig {
    pub common: Common,
    pub modules: Vec<Module>,
}

impl PackConfig {
    pub fn find_multiple_inheritance(&self) -> Result<(), Vec<String>> {
        let package_to_module = self.modules.iter()
            .flat_map(|module|
                module.packages.iter().map(|p| (module.name.clone(), p.clone()))
            ).fold(HashMap::new(), |mut acc, (package, module_name)| {
            acc.entry(module_name)
                .and_modify(|v: &mut HashSet<String>| { v.insert(package.clone()); })
                .or_insert_with(|| {
                    let mut set = HashSet::new();
                    set.insert(package.clone());
                    set
                });
            acc
        })
            ;

        let mut errors = vec![];
        for (package, modules) in package_to_module {
            if modules.len() > 1 {
                let mut pkg_str = String::new();
                for pkg in modules {
                    pkg_str.push_str(&format!(", `{}`", pkg));
                }
                if pkg_str.len() >= 2 {
                    pkg_str.remove(0);
                    pkg_str.remove(0);
                }
                let error_msg = format!("Package `{}` is used by multiple modules: {}", package, pkg_str);
                errors.push(error_msg);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Common {
    pub version: String,
    #[serde(default = "default_bool_false")]
    pub overwrite: bool,
    #[serde(default = "String::new")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub author: String,
    #[serde(default = "String::new")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub publisher: String,
    #[serde(default = "String::new")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub license: String,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default = "default_engine")]
    pub engines: HashMap<String, String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name_prefix: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub display_name_prefix: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub icon_dir: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub icon: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub repository: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Module {
    pub name: String,
    pub display_name: String,
    #[serde(default = "String::new")]
    pub description: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default = "Vec::new")]
    pub categories: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default = "Vec::new")]
    pub activation_events: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default = "Vec::new")]
    pub packages: Vec<String>,
}


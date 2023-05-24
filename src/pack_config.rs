use std::collections::HashMap;
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


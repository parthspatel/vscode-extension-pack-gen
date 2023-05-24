use std::collections::HashMap;
use std::path::{Path, PathBuf};
use image::{GenericImageView, ImageFormat, Pixel, RgbaImage};
use serde::{Deserialize, Deserializer, Serialize};

fn default_engine() -> HashMap<String, String> { HashMap::from([("vscode".to_string(), " ^ 1.78.0".to_string())]) }

fn default_bool_false() -> bool { false }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Packages {
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



pub fn overlay_icon(output_path:&PathBuf, base_path: &PathBuf, overlay_path: &PathBuf, ) {
    let base_image = image::open(base_path).expect("Failed to open base image.");
    let overlay_image = image::open(overlay_path).expect("Failed to open overlay image.");

    let base_width = base_image.width();
    let base_height = base_image.height();

    let overlay_size = std::cmp::min(base_width, base_height) / 4;

    let resized_overlay = overlay_image.resize_exact(
        overlay_size,
        overlay_size,
        image::imageops::FilterType::Lanczos3,
    );

    let overlay_width = resized_overlay.width();
    let overlay_height = resized_overlay.height();

    let padding_percent = 0.05;
    let padding_x = (base_width as f32 * padding_percent) as u32;
    let padding_y = (base_height as f32 * padding_percent) as u32;

    let overlay_x = base_width - overlay_width - padding_x;
    let overlay_y = base_height - overlay_height - padding_y;

    let mut result_image = RgbaImage::from_fn(base_width, base_height, |x, y| {
        let base_pixel = base_image.get_pixel(x, y).to_rgba();
        let overlay_pixel = resized_overlay.get_pixel(x - overlay_x, y - overlay_y).to_rgba();
        if overlay_pixel[3] == 0 {
            base_pixel
        } else {
            overlay_pixel
        }
    });

    result_image
        .save_with_format("result.png", ImageFormat::Png)
        .expect("Failed to save result image.");
}

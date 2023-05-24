use image::{DynamicImage, GenericImageView, ImageFormat, Pixel, Rgba, RgbaImage};
use std::cmp::{max, min};
use std::path::PathBuf;
use crate::clone_with::CloneWith;

pub enum OverlayPosition {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

#[derive(Debug, CloneWith)]
pub struct IconOverlay {
    pub overlay_position: OverlayPosition,
    pub padding_percent: f32,
}

impl Default for IconOverlay {
    fn default() -> Self {
        IconOverlay {
            overlay_position: OverlayPosition::BottomRight,
            padding_percent: 0.05,
        }
    }
}

impl IconOverlay {
    pub fn new() -> Self {
        IconOverlay::default()
    }

    pub fn overlay(
        &self,
        base_path: &PathBuf,
        overlay_path: &PathBuf,
        output_path: &PathBuf,
    ) -> Result<(), String> {
        let base_image = image::open(base_path).map_err(|e| format!("Failed to open base image: {}", e))?;
        let overlay_image =
            image::open(overlay_path).map_err(|e| format!("Failed to open overlay image: {}", e))?;

        let base_width = base_image.width();
        let base_height = base_image.height();

        let overlay_size = min(base_width, base_height) / 4;

        let resized_overlay = overlay_image.resize_exact(
            overlay_size,
            overlay_size,
            image::imageops::FilterType::Lanczos3,
        );

        let overlay_width = resized_overlay.width();
        let overlay_height = resized_overlay.height();

        let padding_x = (base_width as f32 * self.padding_percent) as u32;
        let padding_y = (base_height as f32 * self.padding_percent) as u32;

        let (overlay_x, overlay_y) = match self.overlay_position {
            OverlayPosition::TopRight => (base_width - overlay_width - padding_x, padding_y),
            OverlayPosition::TopLeft => (padding_x, padding_y),
            OverlayPosition::BottomRight => (
                base_width - overlay_width - padding_x,
                base_height - overlay_height - padding_y,
            ),
            OverlayPosition::BottomLeft => (padding_x, base_height - overlay_height - padding_y),
        };

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
            .save_with_format(output_path, ImageFormat::Png)
            .map_err(|e| format!("Failed to save result image: {}", e))?;

        Ok(())
    }
}

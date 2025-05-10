// src/assets/icon_loader.rs

use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    reflect::TypePath,
};
use regex::Regex;
use thiserror::Error;

#[derive(Asset, TypePath, Debug)]
pub struct Icon {
    pub data: Vec<u8>,
}

#[derive(Default)]
pub struct IconLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum IconLoaderError {
    #[error("Konnte Icon nicht laden: {0}")]
    Io(#[from] std::io::Error),
}

impl AssetLoader for IconLoader {
    type Asset = Image;
    type Settings = ();
    type Error = IconLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        match convert_svg_to_image(&bytes, 32, 32) {
            Some(image) => Ok(image),
            None => Err(IconLoaderError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to convert svg to image",
            ))),
        }
    }

    fn extensions(&self) -> &[&str] {
        &["svg"]
    }
}

fn convert_svg_to_image(svg_data: &[u8], target_width: u32, target_height: u32) -> Option<Image> {
    // String aus Bytes
    let mut svg_string = std::str::from_utf8(svg_data).ok()?.to_string();

    // Regex: matcht fill="…"
    let re = Regex::new(r#"fill="[^"]*""#).unwrap();
    // Ersetze alle Vorkommen durch fill="deine_farbe"
    svg_string = re
        .replace_all(&svg_string, format!(r#"fill="{}""#, "#FFFFFF").as_str())
        .into_owned();

    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_str(&svg_string, &opt).ok()?;

    let orig_width = tree.size().width() as f32;
    let orig_height = tree.size().height() as f32;
    let scale_x = target_width as f32 / orig_width;
    let scale_y = target_height as f32 / orig_height;
    let mut pixmap = tiny_skia::Pixmap::new(target_width, target_height)?;
    let transform = tiny_skia::Transform::from_scale(scale_x, scale_y);
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    let image = Image::new(
        bevy::render::render_resource::Extent3d {
            width: pixmap.width(),
            height: pixmap.height(),
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        pixmap.data().to_vec(),
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        bevy::asset::RenderAssetUsages::RENDER_WORLD,
    );
    Some(image)
}

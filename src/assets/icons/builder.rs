use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use regex::Regex;
use std::path::Path;
use tiny_skia::{ColorU8, Pixmap, PremultipliedColor, Transform};

pub struct IconBuilder;

impl IconBuilder {
    const DEFAULT_SIZE: f32 = 32.0;

    pub fn new(svg_data: &str, size: Option<f32>, color: Option<Color>) -> Option<Image> {
        let size = size.unwrap_or(Self::DEFAULT_SIZE);
        let color = color.unwrap_or(Color::WHITE);

        // SVG mit gewünschter Füllfarbe anpassen
        let colored_svg = Self::apply_fill(svg_data, color);
        // SVG parsen
        let tree = Self::load_svg_tree(&colored_svg)?;
        // SVG in Pixmap rendern
        let pixmap = Self::render_tree_to_pixmap(&tree, size)?;
        // Pixmap in gerades RGBA extrahieren
        let rgba_data = Self::extract_rgba(&pixmap);

        // Bevy-Image erstellen
        let image = Self::build_image(rgba_data, pixmap.width(), pixmap.height());
        // PNG-Debug nur in Debug Builds
        #[cfg(debug_assertions)]
        let _ = save_debug_png(&image, "icon_debug.png");

        Some(image)
    }

    fn apply_fill(svg: &str, color: Color) -> String {
        let mut hex = color
            .to_srgba()
            .to_hex()
            .trim_start_matches('#')
            .to_string();
        let mut alpha = 1.0;
        if hex.len() == 8 {
            if let Ok(a) = u8::from_str_radix(&hex[6..], 16) {
                alpha = (a as f32) / 255.0;
            }
            hex.truncate(6);
        }
        let fill_attr = format!("#{}", hex);
        let re = Regex::new(r#"fill="[^"]*""#).unwrap();

        re.replace_all(svg, |_: &regex::Captures| {
            format!(r#"fill="{}" fill-opacity="{}""#, fill_attr, alpha)
        })
        .to_string()
    }

    fn load_svg_tree(svg: &str) -> Option<usvg::Tree> {
        usvg::Tree::from_str(svg, &usvg::Options::default()).ok()
    }

    fn render_tree_to_pixmap(tree: &usvg::Tree, size: f32) -> Option<Pixmap> {
        let (w, h) = (size as u32, size as u32);
        let scale = size / tree.size().width() as f32;
        let mut pixmap = Pixmap::new(w, h)?;
        resvg::render(
            tree,
            Transform::from_scale(scale, scale),
            &mut pixmap.as_mut(),
        );
        Some(pixmap)
    }

    fn extract_rgba(pixmap: &Pixmap) -> Vec<u8> {
        let mut data = Vec::with_capacity((pixmap.width() * pixmap.height() * 4) as usize);
        for pixel in pixmap.pixels() {
            let straight = pixel.demultiply();
            let cu8: ColorU8 = straight;
            data.push(cu8.red());
            data.push(cu8.green());
            data.push(cu8.blue());
            data.push(cu8.alpha());
        }
        data
    }

    fn build_image(data: Vec<u8>, width: u32, height: u32) -> Image {
        Image::new(
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            data,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        )
    }
}

#[cfg(debug_assertions)]
fn save_debug_png(image: &Image, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dyn_img = image.clone().try_into_dynamic()?.to_rgba8();
    dyn_img.save(Path::new(path))?;
    Ok(())
}

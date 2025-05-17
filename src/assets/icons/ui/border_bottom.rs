use super::super::{
    resource::{IconCache, IconKey},
    IconBuilder,
};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

const ICON: &str = "<svg width=\"15\" height=\"15\" viewBox=\"0 0 15 15\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">
  <path fill-rule=\"evenodd\" clip-rule=\"evenodd\" d=\"M1 13.25L14 13.25V14.75L1 14.75V13.25Z\" fill=\"currentColor\" />
  <rect x=\"7\" y=\"5\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"13\" y=\"5\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"7\" y=\"3\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"13\" y=\"3\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"7\" y=\"7\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"7\" y=\"1\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"13\" y=\"7\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"13\" y=\"1\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"5\" y=\"7\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"5\" y=\"1\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"3\" y=\"7\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"3\" y=\"1\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"9\" y=\"7\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"9\" y=\"1\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"11\" y=\"7\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"11\" y=\"1\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"7\" y=\"9\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"13\" y=\"9\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"7\" y=\"11\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"13\" y=\"11\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"1\" y=\"5\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"1\" y=\"3\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"1\" y=\"7\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"1\" y=\"1\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"1\" y=\"9\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"1\" y=\"11\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
</svg>
";

#[derive(SystemParam)]
pub struct BorderBottomIcon<'w> {
    cache: ResMut<'w, IconCache>,
    images: ResMut<'w, Assets<Image>>,
}

impl<'w> BorderBottomIcon<'w> {
    pub fn spawn(&mut self, size: f32, color: Color) -> Handle<Image> {
        let key = IconKey {
            size: size as u32,
            color: [
                color.to_srgba().to_vec4()[0] as u8,
                color.to_srgba().to_vec4()[1] as u8,
                color.to_srgba().to_vec4()[2] as u8,
                color.to_srgba().to_vec4()[3] as u8,
            ],
        };
        if let Some(handle) = self.cache.0.get(&key) {
            handle.clone()
        } else {
            let image = IconBuilder::new(ICON, Some(size), Some(color)).unwrap();
            let handle = self.images.add(image);
            self.cache.0.insert(key, handle.clone());
            handle
        }
    }
}

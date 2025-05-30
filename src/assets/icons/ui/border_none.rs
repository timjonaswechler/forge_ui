use super::super::{
    resource::{IconCache, IconKey},
    IconBuilder,
};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

const ICON: &str = "<svg width=\"15\" height=\"15\" viewBox=\"0 0 15 15\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">
  <rect x=\"7\" y=\"5.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"13\" y=\"5.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"7\" y=\"3.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"13\" y=\"3.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"7\" y=\"7.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"7\" y=\"13.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"7\" y=\"1.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"13\" y=\"7.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"13\" y=\"13.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"13\" y=\"1.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"5\" y=\"7.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"5\" y=\"13.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"5\" y=\"1.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"3\" y=\"7.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"3\" y=\"13.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"3\" y=\"1.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"9\" y=\"7.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"9\" y=\"13.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"9\" y=\"1.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"11\" y=\"7.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"11\" y=\"13.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"11\" y=\"1.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"7\" y=\"9.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"13\" y=\"9.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"7\" y=\"11.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"13\" y=\"11.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"1\" y=\"5.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"1\" y=\"3.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"1\" y=\"7.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"1\" y=\"13.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"1\" y=\"1.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"1\" y=\"9.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
  <rect x=\"1\" y=\"11.025\" width=\"1\" height=\"1\" rx=\".5\" fill=\"currentColor\" />
</svg>
";

#[derive(SystemParam)]
pub struct BorderNoneIcon<'w> {
    cache: ResMut<'w, IconCache>,
    images: ResMut<'w, Assets<Image>>,
}

impl<'w> BorderNoneIcon<'w> {
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

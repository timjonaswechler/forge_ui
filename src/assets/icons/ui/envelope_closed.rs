use super::super::{
    resource::{IconCache, IconKey},
    IconBuilder,
};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

const ICON: &str = "<svg width=\"15\" height=\"15\" viewBox=\"0 0 15 15\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">
  <path
    fill-rule=\"evenodd\"
    clip-rule=\"evenodd\"
    d=\"M1 2C0.447715 2 0 2.44772 0 3V12C0 12.5523 0.447715 13 1 13H14C14.5523 13 15 12.5523 15 12V3C15 2.44772 14.5523 2 14 2H1ZM1 3L14 3V3.92494C13.9174 3.92486 13.8338 3.94751 13.7589 3.99505L7.5 7.96703L1.24112 3.99505C1.16621 3.94751 1.0826 3.92486 1 3.92494V3ZM1 4.90797V12H14V4.90797L7.74112 8.87995C7.59394 8.97335 7.40606 8.97335 7.25888 8.87995L1 4.90797Z\"
    fill=\"currentColor\"
  />
</svg>
";

#[derive(SystemParam)]
pub struct EnvelopeClosedIcon<'w> {
    cache: ResMut<'w, IconCache>,
    images: ResMut<'w, Assets<Image>>,
}

impl<'w> EnvelopeClosedIcon<'w> {
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

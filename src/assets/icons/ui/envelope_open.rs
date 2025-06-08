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
    d=\"M7.94721 0.164594C7.66569 0.0238299 7.33431 0.0238302 7.05279 0.164594L0.552786 3.41459C0.214002 3.58399 0 3.93025 0 4.30902V12C0 12.5523 0.447715 13 1 13H14C14.5523 13 15 12.5523 15 12V4.30902C15 3.93025 14.786 3.58399 14.4472 3.41459L7.94721 0.164594ZM13.5689 4.09349L7.5 1.05902L1.43105 4.09349L7.5 7.29136L13.5689 4.09349ZM1 4.88366V12H14V4.88366L7.70977 8.19813C7.57848 8.26731 7.42152 8.26731 7.29023 8.19813L1 4.88366Z\"
    fill=\"currentColor\"
  />
</svg>
";

#[derive(SystemParam)]
pub struct EnvelopeOpenIcon<'w> {
    cache: ResMut<'w, IconCache>,
    images: ResMut<'w, Assets<Image>>,
}

impl<'w> EnvelopeOpenIcon<'w> {
    pub fn new(&mut self, size: f32, color: Color) -> Handle<Image> {
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

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
    d=\"M4.79993 3.50017C4.79993 3.25164 5.0014 3.05017 5.24993 3.05017H9.74993C9.99845 3.05017 10.1999 3.25164 10.1999 3.50017C10.1999 3.7487 9.99845 3.95017 9.74993 3.95017H8.09993V11.05H9.74994C9.99847 11.05 10.1999 11.2515 10.1999 11.5C10.1999 11.7486 9.99847 11.95 9.74994 11.95H5.24994C5.00141 11.95 4.79994 11.7486 4.79994 11.5C4.79994 11.2515 5.00141 11.05 5.24994 11.05H6.89993V3.95017H5.24993C5.0014 3.95017 4.79993 3.7487 4.79993 3.50017Z\"
    fill=\"currentColor\"
  />
</svg>
";

#[derive(SystemParam)]
pub struct FontRomanIcon<'w> {
    cache: ResMut<'w, IconCache>,
    images: ResMut<'w, Assets<Image>>,
}

impl<'w> FontRomanIcon<'w> {
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

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
    d=\"M1.5 2H13.5C13.7761 2 14 2.22386 14 2.5V7H1V2.5C1 2.22386 1.22386 2 1.5 2ZM1 8V12.5C1 12.7761 1.22386 13 1.5 13H13.5C13.7761 13 14 12.7761 14 12.5V8H1ZM0 2.5C0 1.67157 0.671573 1 1.5 1H13.5C14.3284 1 15 1.67157 15 2.5V12.5C15 13.3284 14.3284 14 13.5 14H1.5C0.671573 14 0 13.3284 0 12.5V2.5Z\"
    fill=\"currentColor\"
  />
</svg>
";

#[derive(SystemParam)]
pub struct ViewHorizontalIcon<'w> {
    cache: ResMut<'w, IconCache>,
    images: ResMut<'w, Assets<Image>>,
}

impl<'w> ViewHorizontalIcon<'w> {
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

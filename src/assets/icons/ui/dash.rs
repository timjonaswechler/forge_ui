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
    d=\"M5 7.5C5 7.22386 5.22386 7 5.5 7H9.5C9.77614 7 10 7.22386 10 7.5C10 7.77614 9.77614 8 9.5 8H5.5C5.22386 8 5 7.77614 5 7.5Z\"
    fill=\"currentColor\"
  />
</svg>
";

#[derive(SystemParam)]
pub struct DashIcon<'w> {
    cache: ResMut<'w, IconCache>,
    images: ResMut<'w, Assets<Image>>,
}

impl<'w> DashIcon<'w> {
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

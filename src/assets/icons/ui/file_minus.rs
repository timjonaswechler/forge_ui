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
    d=\"M3 2.5C3 2.22386 3.22386 2 3.5 2H9.29289L12 4.70711V12.5C12 12.7761 11.7761 13 11.5 13H3.5C3.22386 13 3 12.7761 3 12.5V2.5ZM3.5 1C2.67157 1 2 1.67157 2 2.5V12.5C2 13.3284 2.67157 14 3.5 14H11.5C12.3284 14 13 13.3284 13 12.5V4.60355C13 4.40464 12.921 4.21388 12.7803 4.07322L9.85355 1.14645C9.75979 1.05268 9.63261 1 9.5 1H3.5ZM5.25 7C4.97386 7 4.75 7.22386 4.75 7.5C4.75 7.77614 4.97386 8 5.25 8H9.75C10.0261 8 10.25 7.77614 10.25 7.5C10.25 7.22386 10.0261 7 9.75 7H5.25Z\"
    fill=\"currentColor\"
  />
</svg>
";

#[derive(SystemParam)]
pub struct FileMinusIcon<'w> {
    cache: ResMut<'w, IconCache>,
    images: ResMut<'w, Assets<Image>>,
}

impl<'w> FileMinusIcon<'w> {
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

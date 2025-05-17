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
    d=\"M3.64645 3.64644C3.45118 3.8417 3.45118 4.15828 3.64645 4.35354L10.2929 11L6 11C5.72386 11 5.5 11.2239 5.5 11.5C5.5 11.7761 5.72386 12 6 12L11.5 12C11.6326 12 11.7598 11.9473 11.8536 11.8536C11.9473 11.7598 12 11.6326 12 11.5L12 5.99999C12 5.72385 11.7761 5.49999 11.5 5.49999C11.2239 5.49999 11 5.72385 11 5.99999V10.2929L4.35355 3.64643C4.15829 3.45117 3.84171 3.45117 3.64645 3.64644Z\"
    fill=\"currentColor\"
  />
</svg>
";

#[derive(SystemParam)]
pub struct ArrowBottomRightIcon<'w> {
    cache: ResMut<'w, IconCache>,
    images: ResMut<'w, Assets<Image>>,
}

impl<'w> ArrowBottomRightIcon<'w> {
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

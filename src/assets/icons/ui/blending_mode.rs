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
    d=\"M3 9C3 6.5 4.5 4.25 7.5 1.5C10.5 4.25 12 6.5 12 9C12 11.4853 9.98528 13.5 7.5 13.5C5.01472 13.5 3 11.4853 3 9ZM10.9524 8.30307C9.67347 7.82121 8.2879 8.46208 6.98956 9.06259C5.9327 9.55142 4.93365 10.0135 4.09695 9.82153C4.03357 9.55804 4 9.28294 4 9C4 7.11203 5.02686 5.27195 7.5 2.87357C9.66837 4.97639 10.725 6.65004 10.9524 8.30307Z\"
    fill=\"currentColor\"
  />
</svg>
";

#[derive(SystemParam)]
pub struct BlendingModeIcon<'w> {
    cache: ResMut<'w, IconCache>,
    images: ResMut<'w, Assets<Image>>,
}

impl<'w> BlendingModeIcon<'w> {
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

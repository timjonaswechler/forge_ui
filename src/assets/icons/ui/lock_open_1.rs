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
    d=\"M7.4986 0C6.3257 0 5.36107 0.38943 4.73753 1.19361C4.23745 1.83856 4 2.68242 4 3.63325H5C5 2.84313 5.19691 2.23312 5.5278 1.80636C5.91615 1.30552 6.55152 1 7.4986 1C8.35683 1 8.96336 1.26502 9.35846 1.68623C9.75793 2.11211 10 2.76044 10 3.63601V6H3C2.44772 6 2 6.44772 2 7V13C2 13.5523 2.44772 14 3 14H12C12.5523 14 13 13.5523 13 13V7C13 6.44771 12.5523 6 12 6H11V3.63601C11 2.58135 10.7065 1.66167 10.0878 1.0021C9.46477 0.337871 8.57061 0 7.4986 0ZM3 7H12V13H3V7Z\"
    fill=\"currentColor\"
  />
</svg>
";

#[derive(SystemParam)]
pub struct LockOpen1Icon<'w> {
    cache: ResMut<'w, IconCache>,
    images: ResMut<'w, Assets<Image>>,
}

impl<'w> LockOpen1Icon<'w> {
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

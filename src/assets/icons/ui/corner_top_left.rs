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
    d=\"M9.87737 3H9.9H11.5C11.7761 3 12 3.22386 12 3.5C12 3.77614 11.7761 4 11.5 4H9.9C8.77164 4 7.95545 4.00039 7.31352 4.05284C6.67744 4.10481 6.25662 4.20539 5.91103 4.38148C5.25247 4.71703 4.71703 5.25247 4.38148 5.91103C4.20539 6.25662 4.10481 6.67744 4.05284 7.31352C4.00039 7.95545 4 8.77164 4 9.9V11.5C4 11.7761 3.77614 12 3.5 12C3.22386 12 3 11.7761 3 11.5V9.9V9.87737C3 8.77641 3 7.91948 3.05616 7.23209C3.11318 6.53416 3.23058 5.9671 3.49047 5.45704C3.9219 4.61031 4.61031 3.9219 5.45704 3.49047C5.9671 3.23058 6.53416 3.11318 7.23209 3.05616C7.91948 3 8.77641 3 9.87737 3Z\"
    fill=\"currentColor\"
  />
</svg>
";

#[derive(SystemParam)]
pub struct CornerTopLeftIcon<'w> {
    cache: ResMut<'w, IconCache>,
    images: ResMut<'w, Assets<Image>>,
}

impl<'w> CornerTopLeftIcon<'w> {
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

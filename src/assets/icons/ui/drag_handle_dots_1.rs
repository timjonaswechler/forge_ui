use super::super::{
    resource::{IconCache, IconKey},
    IconBuilder,
};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

const ICON: &str = "<svg width=\"15\" height=\"15\" viewBox=\"0 0 15 15\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">
  <circle cx=\"4.5\" cy=\"2.5\" r=\".6\" fill=\"currentColor\" />
  <circle cx=\"4.5\" cy=\"4.5\" r=\".6\" fill=\"currentColor\" />
  <circle cx=\"4.5\" cy=\"6.499\" r=\".6\" fill=\"currentColor\" />
  <circle cx=\"4.5\" cy=\"8.499\" r=\".6\" fill=\"currentColor\" />
  <circle cx=\"4.5\" cy=\"10.498\" r=\".6\" fill=\"currentColor\" />
  <circle cx=\"4.5\" cy=\"12.498\" r=\".6\" fill=\"currentColor\" />
  <circle cx=\"6.5\" cy=\"2.5\" r=\".6\" fill=\"currentColor\" />
  <circle cx=\"6.5\" cy=\"4.5\" r=\".6\" fill=\"currentColor\" />
  <circle cx=\"6.5\" cy=\"6.499\" r=\".6\" fill=\"currentColor\" />
  <circle cx=\"6.5\" cy=\"8.499\" r=\".6\" fill=\"currentColor\" />
  <circle cx=\"6.5\" cy=\"10.498\" r=\".6\" fill=\"currentColor\" />
  <circle cx=\"6.5\" cy=\"12.498\" r=\".6\" fill=\"currentColor\" />
  <circle cx=\"8.499\" cy=\"2.5\" r=\".6\" fill=\"currentColor\" />
  <circle cx=\"8.499\" cy=\"4.5\" r=\".6\" fill=\"currentColor\" />
  <circle cx=\"8.499\" cy=\"6.499\" r=\".6\" fill=\"currentColor\" />
  <circle cx=\"8.499\" cy=\"8.499\" r=\".6\" fill=\"currentColor\" />
  <circle cx=\"8.499\" cy=\"10.498\" r=\".6\" fill=\"currentColor\" />
  <circle cx=\"8.499\" cy=\"12.498\" r=\".6\" fill=\"currentColor\" />
  <circle cx=\"10.499\" cy=\"2.5\" r=\".6\" fill=\"currentColor\" />
  <circle cx=\"10.499\" cy=\"4.5\" r=\".6\" fill=\"currentColor\" />
  <circle cx=\"10.499\" cy=\"6.499\" r=\".6\" fill=\"currentColor\" />
  <circle cx=\"10.499\" cy=\"8.499\" r=\".6\" fill=\"currentColor\" />
  <circle cx=\"10.499\" cy=\"10.498\" r=\".6\" fill=\"currentColor\" />
  <circle cx=\"10.499\" cy=\"12.498\" r=\".6\" fill=\"currentColor\" />
</svg>
";

#[derive(SystemParam)]
pub struct DragHandleDots1Icon<'w> {
    cache: ResMut<'w, IconCache>,
    images: ResMut<'w, Assets<Image>>,
}

impl<'w> DragHandleDots1Icon<'w> {
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

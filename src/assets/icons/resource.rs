use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub(crate) struct IconKey {
    pub size: u32,
    pub color: [u8; 4], // RGBA representation as u8
}

impl IconKey {
    pub(crate) fn new(size: u32, color: Color) -> Self {
        let rgba = color.to_srgba().to_vec4();
        let color = [
            (rgba[0] * 255.0).round() as u8,
            (rgba[1] * 255.0).round() as u8,
            (rgba[2] * 255.0).round() as u8,
            (rgba[3] * 255.0).round() as u8,
        ];
        Self { size, color }
    }
}

#[derive(Resource, Default)]
pub struct IconCache(pub HashMap<IconKey, Handle<Image>>);

// src/assets/icon_loader.rs
use bevy::asset::{io::Reader, AssetLoader, LoadContext};
use bevy::{prelude::*, reflect::TypePath};
use thiserror::Error;

#[derive(Asset, TypePath, Debug)]
pub struct Icon {
    pub data: Vec<u8>,
}

#[derive(Default)]
pub struct IconLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum IconLoaderError {
    #[error("Konnte Icon nicht laden: {0}")]
    Io(#[from] std::io::Error),
}

impl AssetLoader for IconLoader {
    type Asset = Icon;
    type Settings = ();
    type Error = IconLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        Ok(Icon { data: bytes })
    }

    fn extensions(&self) -> &[&str] {
        &["svg"]
    }
}

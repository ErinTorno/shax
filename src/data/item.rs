use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::{Serialize, Deserialize};

use crate::data::sprite::*;

#[derive(Clone, Deserialize, Debug, Serialize)]
pub enum Slot {
    Hand,
    Crown,
    Consumable,
    Dungeon,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ItemConfig {
    pub id:     String,
    pub name:   String,
    pub sprite: SpriteConfig,
    pub slot:   Slot,
}

#[derive(Clone, Debug, TypeUuid)]
#[uuid = "c0643fe7-4945-4b18-9957-55f9edb87dfc"]
pub struct Item {
    pub id:     String,
    pub name:   String,
    pub sprite: Handle<SpriteInfo>,
    pub slot:   Slot,
}

#[derive(Default)]
pub struct ItemLoader;

impl AssetLoader for ItemLoader {
    fn extensions(&self) -> &[&str] { &["item.ron"] }

    fn load<'a>(&'a self, bytes: &'a [u8], load_context: &'a mut LoadContext) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let config = ron::de::from_bytes::<ItemConfig>(bytes)?;
            let mut dependencies = Vec::new();
            let sprite = config.sprite.to_sprite_info(load_context, &mut dependencies);
            let sprite_handle = load_context.set_labeled_asset("sprite", LoadedAsset::new(sprite));

            load_context.set_default_asset(LoadedAsset::new(Item {
                id:     config.id,
                name:   config.name,
                slot:   config.slot,
                sprite: sprite_handle,
            }).with_dependencies(dependencies));
            Ok(())
        })
    }
}
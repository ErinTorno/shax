use bevy::{
    asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::{Serialize, Deserialize};

use crate::data::sprite::*;
use crate::lua::*;
use crate::util::types::*;

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct PrefabConfig {
    pub sprite: SpriteConfig,
    pub script: Option<Embeddable<String>>,
}

#[derive(Clone, Debug, TypeUuid)]
#[uuid = "ae3e9e0a-0f2c-4acb-a959-8becff99b7e1"]
pub struct Prefab {
    pub sprite: Handle<SpriteInfo>,
    pub script: Option<Handle<LuaScript>>,
}

#[derive(Clone, Debug)]
pub struct PrefabToSpawn {
    pub prefab:      Handle<Prefab>,
    pub translation: Vec3,
}

#[derive(Default)]
pub struct PrefabLoader;

impl AssetLoader for PrefabLoader {
    fn extensions(&self) -> &[&str] { &["prefab.ron"] }

    fn load<'a>(&'a self, bytes: &'a [u8], load_context: &'a mut LoadContext) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let prefab_config = ron::de::from_bytes::<PrefabConfig>(bytes)?;
            let mut dependencies = Vec::new();
            let sprite = prefab_config.sprite.to_sprite_info(load_context, &mut dependencies);
            let sprite_handle = load_context.set_labeled_asset("sprite", LoadedAsset::new(sprite));

            let script = {
                if let Some(embeddable) = prefab_config.script {
                    match embeddable {
                        Embeddable::Embedded(s) => {
                            Some(load_context.set_labeled_asset("embedded_script", LoadedAsset::new(LuaScript(s.into_bytes()))))
                        },
                        Embeddable::File(f) => {
                            let path = AssetPath::new(load_context.path().parent().unwrap().join(f), None);
                            dependencies.push(path.clone());
                            Some(load_context.get_handle(path))
                        },
                    }
                } else {
                    None
                }
            };

            load_context.set_default_asset(LoadedAsset::new(Prefab {
                sprite: sprite_handle,
                script,
            }).with_dependencies(dependencies));
            Ok(())
        })
    }
}
use bevy::prelude::*;
use bevy_ldtk::*;

use data::action::*;
use data::item::*;
use data::level::*;
use data::prefab::*;
use data::sprite::*;
use data::turn::*;
use lua::script::*;
use system::action::*;
use system::camera::*;
use system::level::*;
use system::prefab::*;
use system::sprite::*;
use system::turn::*;

mod data;
mod lua;
mod system;
mod util;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_asset::<LuaScript>()
        .add_asset::<Prefab>()
        .add_asset::<SpriteInfo>()
        .init_asset_loader::<ItemLoader>()
        .init_asset_loader::<LuaScriptLoader>()
        .init_asset_loader::<PrefabLoader>()
        .init_resource::<LuaResource>()
        .init_resource::<TurnCount>()
        .insert_resource(MapScale(6.))
        .insert_resource(ControlSettings::default())
        .add_startup_system(setup.system())
        .add_system(load_level.system())
        .add_system(spawn_prefab.system())
        .add_system(update_actions.system())
        .add_system(update_animations.system())
        .add_system(update_camera.system())
        .add_system(update_turn.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_scale:    Res<MapScale>,
) {
    // Enable hot reload
    asset_server.watch_for_changes().unwrap();
    //asset_server.load_folder(".").expect("Error loading assets folder");

    commands
        .spawn()
        .insert_bundle(LdtkMapBundle {
            map: asset_server.load("world.ldtk"),
            config: LdtkMapConfig {
                set_clear_color: true,
                scale: map_scale.0,
                level: 0,
                center_map: false,
            },
            ..Default::default()
        })
        .insert(LevelToLoad(0));
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
}
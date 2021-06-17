use bevy::prelude::*;
use bevy_ldtk::*;
use ldtk::*;
use serde_json::value::Value;
use std::collections::HashMap;

use crate::data::action::*;
use crate::data::level::*;
use crate::data::player::Player;
use crate::data::prefab::*;
use crate::lua::*;

pub fn load_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_scale:    Res<MapScale>,
    map_assets:   Res<Assets<LdtkMap>>,
    mut lua:      ResMut<LuaResource>,
    query_set: QuerySet<(
        Query<(Entity, &Handle<LdtkMap>, &LevelToLoad)>,
    )>, 
) {
    query_set.q0().for_each(|(layer_entity, ldtk_handle, LevelToLoad(level_idx))| {
        if let Some(ltdk_map) = map_assets.get(ldtk_handle) {
            let mut level_info = LevelInfo {level_idx: level_idx.clone(), ..LevelInfo::default()};
            let level = &ltdk_map.project.levels[level_idx.clone()];
            let entity_z = level.layer_instances.as_ref().unwrap().len() as f32 + 1.;
            let mut grid = create_grid(level);

            for field in level.field_instances.iter() {
                match field.__identifier.as_str() {
                    "embedded_script" => lua.exec_script(field.__value.as_str().expect("embedded_script was not a String")),
                    "subtitle"        => level_info.subtitle = field.__value.as_str().map(|s| s.to_string()),
                    "title"           => level_info.title    = field.__value.as_str().expect("title was not a String").to_string(),
                    e => println!("Unhandled level field `{:?}`", e),
                }
            }
            
            level.layer_instances.as_ref().unwrap().iter()
                .filter(|l| l.__type == "Entities")
                .for_each(|layer| {
                    let layer_z = get_layer_z(layer);
                    for entity in &layer.entity_instances {
                        match entity.__identifier.as_str() {
                            "Player_spawn" => {
                                let pos = grid_pos(layer, layer_z, entity);
                                println!("player pos {:?}", pos);
                                // change later
                                let entity = commands.spawn()
                                    .insert(Player)
                                    .insert(LocalActions::default())
                                    .insert(OwningLevel(layer_entity))
                                    .insert(PrefabToSpawn {
                                        prefab: asset_server.load("actors/player.prefab.ron"),
                                        translation: entity_translation(entity, &map_scale, entity_z),
                                    })
                                    .insert(pos)
                                    .id();
                                grid.set(&pos, PosState::Entity(entity));
                            },
                            "Prefab" => {
                                let mut prefab_file = None;
                                for inst in &entity.field_instances {
                                    match inst.__identifier.as_str() {
                                        "prefab" => prefab_file = inst.__value.as_str(),
                                        _ => (),
                                    }
                                }
                                let pos = grid_pos(layer, layer_z, entity);
                                println!("prefab pos {:?}", pos);
                                let entity = commands.spawn()
                                    .insert(OwningLevel(layer_entity))
                                    .insert(PrefabToSpawn {
                                        prefab: asset_server.load(prefab_file.expect("Entity Prefab is missing prefab field")),
                                        translation: entity_translation(entity, &map_scale, entity_z),
                                    })
                                    .insert(pos)
                                    .id();
                                grid.set(&pos, PosState::Entity(entity));
                            },
                            s => panic!("Unknown entity identifier {}", s),
                        }
                    }
                });
            level.layer_instances.as_ref().unwrap().iter()
                .filter(|l| l.__type == "Tiles")
                .for_each(|layer| {
                    let defs = tileset_definition(ltdk_map, layer).expect("Tiles layer missing TileSet definition");
                    println!("for layer `{}` {:?}x{:?} | tile set defs {:?}", layer.__identifier, layer.__c_wid, layer.__c_hei, defs);
                    let layer_z = get_layer_z(layer);

                    let tile_size = layer.__grid_size;

                    for tile in layer.grid_tiles.iter() {
                        let pos = Pos { x: tile.px[0] / tile_size, y: tile.px[1] / tile_size, z: layer_z};
                        if let Some(state) = defs.get(&tile.t) {
                            grid.set(&pos, state.clone());
                        }
                    }
                });
            commands.entity(layer_entity)
                .insert(level_info)
                .insert(grid)
                .remove::<LevelToLoad>();
        }
    });
}

fn get_dim(level: &Level) -> Result<(usize, usize), String> {
    if let Some(layers) = level.layer_instances.as_ref() {
        if !layers.is_empty() {
            let w = layers[0].__c_wid;
            let h = layers[0].__c_hei;
            for layer in layers.iter() {
                if layer.__c_wid != w || layer.__c_hei != h {
                    return Err(format!("Level `{}` error: mismatched layer sizes (expected {:?}x{:?}, found {:?}x{:?} on layer {:?})", level.identifier, w, h, layer.__c_wid, layer.__c_hei, layer.__identifier));
                }
            }
            Ok((w as usize, h as usize))
        } else {
            Err(format!("Level `{}` error: no layer instances", level.identifier))
        }
    } else {
        Err(format!("Level `{}` error: no layer instances", level.identifier))
    }
}

fn entity_translation(entity: &EntityInstance, map_scale: &MapScale, layer: f32) -> Vec3 {
    Vec3::new((entity.px[0] as f32) * map_scale.0, -(entity.px[1] as f32) * map_scale.0, layer as f32)
}

fn create_grid(level: &Level) -> Grid {
    let (w, h) = get_dim(level).unwrap_or_else(|s| panic!(s));
    let mut max_z = 0;
    for layer in level.layer_instances.as_ref().unwrap() {
        let z = get_layer_z(layer);
        max_z = max_z.max(z);
    }
    Grid::new(w, h, (max_z + 1) as usize)
}

fn get_layer_z(layer: &LayerInstance) -> i32 {
    let name = layer.__identifier.clone();
    match &name[name.find("Z").unwrap_or(0) + 1..name.find("_").unwrap_or(name.len())].parse::<i32>() {
        Ok(z) => z.clone(),
        Err(e) => {
            println!("Missing layer Z `{}`: {:?}", layer.__identifier, e);
            0
        },
    }
}

fn grid_pos(layer: &LayerInstance, layer_z: i32, entity: &EntityInstance) -> Pos {
    Pos {
        x: entity.px[0] as i32 / layer.__grid_size as i32,
        y: entity.px[1] as i32 / layer.__grid_size as i32,
        z: layer_z,
    }
}

fn tileset_definition(map: &LdtkMap, layer: &LayerInstance) -> Option<HashMap<i32, PosState>> {
    if let Some(tags) = layer.__tileset_def_uid.map(|tid| &map.project.defs.tilesets.get(tid as usize - 1).expect("Layer has invalid tileset id").enum_tags) {
        let mut tile_info = HashMap::new();
        for tag in tags {
            if let Some(Value::Array(ara)) = tag.get("tileIds") {
                if let Some(Value::String(state_str)) = tag.get("enumValueId") {
                    let state = ron::de::from_str::<PosState>(&state_str).expect("Unable to parse PosState from enum value");
                    for id in ara {
                        if let Some(n) = id.as_i64() {
                            tile_info.insert(n as i32, state.clone());
                        }
                    }
                }
            }
        }
        Some(tile_info)
    } else {
        None
    }
}
use bevy::prelude::*;

use crate::data::level::*;
use crate::lua::{script::*, entity::*};
use crate::data::prefab::*;
use crate::data::sprite::SpriteInfo;
use crate::data::sprite::TILE_SIZE;

pub fn spawn_prefab(
    mut commands: Commands,
    mut lua:      ResMut<LuaResource>,
    map_scale:    Res<MapScale>,
    prefabs:      Res<Assets<Prefab>>,
    scripts:      Res<Assets<LuaScript>>,
    sprites:      Res<Assets<SpriteInfo>>,
    query:        Query<(Entity, &PrefabToSpawn)>,
) {
    query.for_each(|(entity, pref_to_spawn)| {
        if let Some(prefab) = prefabs.get(&pref_to_spawn.prefab) {
            if let Some(sprite) = sprites.get(&prefab.sprite) {
                //let scale = sprite.scale * 1.5 / map_scale.0;  // why 1.5? it's a mystery!
                let color = sprite.anim.default_tint().color();
                let index = sprite.anim.default_index();
                let scale = sprite.scale * 0.5; // why 0.5? it's also a mystery!
                commands.entity(entity)
                    .remove::<PrefabToSpawn>()
                    .insert_bundle(SpriteSheetBundle {
                        texture_atlas: sprite.atlas.clone(),
                        sprite: TextureAtlasSprite { color, index, flip_x: false, flip_y: false },
                        transform: Transform {
                            scale,
                            translation: pref_to_spawn.translation + Vec3::new(0.5 * TILE_SIZE * map_scale.0, -0.5 * TILE_SIZE * map_scale.0, 0.),
                            ..Transform::identity()
                        },
                        global_transform: GlobalTransform {
                            translation: pref_to_spawn.translation,
                            ..Default::default()
                        },
                        ..Default::default()
                    });

                if let Some(anim_state) = sprite.anim.default_anim_state() {
                    commands.entity(entity)
                        .insert(sprite.clone())
                        .insert(anim_state);
                }

                if let Some(script_handle) = &prefab.script {
                    let script = scripts.get(script_handle).expect("script dependency not loaded when prefab is spawned");
                    let run_results = lua.exec_script_with_instance(script, LuaEntity::new(entity)).unwrap();
                    if run_results.events_registered.contains(EntityEvent::OnInit) {
                        lua.run_event(EntityEvent::OnInit, run_results).unwrap()
                            .update_entity(&mut commands);
                    } else {
                        run_results.update_entity(&mut commands);
                    }
                }
            }
        }
    });
}
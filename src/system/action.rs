use bevy::{
    prelude::*,
};
use std::collections::HashMap;
use std::time::Duration;

use crate::data::action::*;
use crate::data::level::*;
use crate::data::player::*;
use crate::data::turn::*;
use crate::lua::*;

pub fn update_actions(
    time:           Res<Time>,
    controls:       Res<ControlSettings>,
    keyboard_input: Res<Input<KeyCode>>,
    mut lua:        ResMut<LuaResource>,
    mut turn_count: ResMut<TurnCount>,
    mut query_set:  QuerySet<(
        Query<(Entity, &mut Pos, &mut LocalActions, &OwningLevel)>,
        Query<(Entity, &mut Grid)>,
        Query<(&Player, &Pos)>,
    )>,
) {
    let timestamp = time.seconds_since_startup();
    let mut move_reqs: HashMap<Entity, HashMap<Entity, (Pos, Dir)>> = HashMap::new();

    query_set.q0_mut().for_each_mut(|(entity, pos, mut actions, OwningLevel(level_entity))| {
        actions.north.update(timestamp, keyboard_input.pressed(controls.north));
        actions.south.update(timestamp, keyboard_input.pressed(controls.south));
        actions.east .update(timestamp, keyboard_input.pressed(controls.east));
        actions.west .update(timestamp, keyboard_input.pressed(controls.west));
        actions.run  .update(timestamp, keyboard_input.pressed(controls.run));
        if actions.run.value {
            actions.move_timer.set_duration(Duration::from_secs_f32(SECONDS_TO_RUN));
        } else {
            actions.move_timer.set_duration(Duration::from_secs_f32(SECONDS_TO_WALK));
        }

        actions.move_timer.tick(time.delta());
        if let Some(dir) = actions.dir(timestamp) {
            if actions.move_timer.finished() {
                move_reqs.entry(level_entity.clone())
                    .or_insert_with(|| HashMap::new())
                    .insert(entity, (pos.clone(), dir));
            }
        }
    });
    let mut move_approves = HashMap::new();
    query_set.q1_mut().for_each_mut(|(level_entity, mut grid)| {
        if let Some(entities) = move_reqs.get(&level_entity) {
            for (entity, (prev_pos, dir)) in entities {
                let target_pos = prev_pos.step(dir.clone());
                // println!("prev_pos {:?} target_pos {:?} prev_state {:?} grid_state {:?} is_blocking {:?}", prev_pos, target_pos, grid.get(prev_pos), grid.get(&target_pos), grid.get(&target_pos).is_blocking());
                if !grid.get(&target_pos).is_blocking() {
                    grid.set(prev_pos, PosState::None);
                    grid.set(&target_pos, PosState::Entity(entity.clone()));
                    move_approves.insert(entity.clone(), target_pos);
                    // player has moved, so we increment the turn count
                    turn_count.0 += 1;
                    lua.global.turn_count += 1;
                    lua.sync();
                }
            }
        }
    });
    query_set.q0_mut().for_each_mut(|(entity, mut pos, mut actions, _)| {
        if let Some(new_pos) = move_approves.get(&entity) {
            actions.move_timer.reset();
            pos.x = new_pos.x;
            pos.y = new_pos.y;
            pos.z = new_pos.z;
        }
    });
}
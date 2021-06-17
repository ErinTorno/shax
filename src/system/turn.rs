use bevy::{
    prelude::*,
};
use enumset::*;

use crate::data::turn::*;
use crate::lua::*;

pub fn update_turn(
    turn_count: Res<TurnCount>,
    mut lua:    ResMut<LuaResource>,
    query_set:  QuerySet<(
        Query<(Entity, &EnumSet<EntityEvent>)>,
    )>,
) {
    if turn_count.is_changed() {
        query_set.q0().for_each(|(entity, event_handlers)| {
            if event_handlers.contains(EntityEvent::OnUpdate) {
                lua.run_event(EntityEvent::OnUpdate, LuaEntity::new(entity)).unwrap();
            }
        });
    }
}
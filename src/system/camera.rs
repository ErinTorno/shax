use bevy::{
    prelude::*,
    render::camera::Camera,
};

use crate::data::level::*;
use crate::data::player::*;
use crate::data::sprite::TILE_SIZE;

pub fn update_camera(
    map_scale:     Res<MapScale>,
    mut query_set: QuerySet<(
        Query<(&Transform, &Player)>,
        Query<(&mut Transform, &Camera)>,
        Query<(&mut Transform, &Pos)>,
    )>,
) {
    query_set.q2_mut().for_each_mut(|(mut transform, pos)| {
        transform.translation.x = map_scale.0 * (TILE_SIZE *  pos.x as f32 + 0.5 * TILE_SIZE);
        transform.translation.y = map_scale.0 * (TILE_SIZE * -pos.y as f32 - 0.5 * TILE_SIZE);
        transform.translation.z = pos.z as f32;
    });

    let mut player_trans = None;
    query_set.q0().for_each(|(transform, _)| {
        player_trans = Some(transform.clone());
    });

    if let Some(t) = player_trans {
        query_set.q1_mut().for_each_mut(|(mut transform, _)| {
            transform.translation = t.translation;
        })
    }
}
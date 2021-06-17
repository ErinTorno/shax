use std::borrow::Borrow;

use bevy::prelude::*;
use crate::data::sprite::*;

pub fn update_animations(
    time: Res<Time>,
    query: Query<(&SpriteInfo, &mut AnimState, &mut TextureAtlasSprite)>,
) {
    query.for_each_mut(|(info, mut state, mut texture)| {
        state.update(info, time.borrow());
        texture.color = state.cur_tint(info).color();
        texture.index = state.cur_index(info);
    });
}
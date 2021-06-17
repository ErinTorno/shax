use bevy::{
    asset::{AssetPath, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
};
use core::time::Duration;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::data::color::*;
use crate::util::serde::deserialize_duration_from_f32;

pub const TILE_SIZE: f32 = 12.;
fn default_scale() -> Vec3 { Vec3::new(1., 1., 1.) }
fn default_dim() -> usize { TILE_SIZE as usize }
fn default_tint() -> Palette { Palette::DevWhite }
fn default_index() -> u32 { 0 }
fn default_anim() -> AnimConfig { AnimConfig::Static { index: default_index(), tint: default_tint() } }
fn default_anim_name() -> String { "idle".to_string() }

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Frame {
    index: u32,
    tint:  Palette,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Animation {
    #[serde(deserialize_with = "deserialize_duration_from_f32")]
    pub delay:  Duration,
    pub frames: Vec<Frame>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub enum AnimConfig {
    Static {
        index: u32,
        tint:  Palette,
    },
    Animated {
        #[serde(default = "default_anim_name")]
        default: String,
        animations: HashMap<String, Animation>,
    }
}

impl AnimConfig {
    pub fn to_anim_info(self) -> AnimInfo {
        match self {
            AnimConfig::Static {index, tint} => AnimInfo::Static {index, tint},
            AnimConfig::Animated {default, animations} => {
                let mut i = 0;
                let mut anim_ids = HashMap::with_capacity(animations.len());
                for key in animations.keys() {
                    anim_ids.insert(key.clone(), i);
                    i += 1;
                }
                let anim_vec = animations.values().map(|a| a.clone()).collect();
                AnimInfo::Animated {
                    def_anim_idx: anim_ids.get(&default).expect("No default animation or animation named `idle` found when loading AnimInfo").clone(),
                    name_to_index: anim_ids,
                    anim_vec
                }
            },
        }
    }
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct SpriteConfig {
    pub atlas:   String,
    pub columns: usize,
    pub rows:    usize,
    #[serde(default = "default_dim")]
    pub width:   usize,
    #[serde(default = "default_dim")]
    pub height:  usize,
    #[serde(default = "default_scale")]
    pub scale:   Vec3,
    #[serde(default = "default_anim")]
    pub anim:    AnimConfig,
}

impl SpriteConfig {
    pub fn to_sprite_info(self, load_context: &mut LoadContext, dependencies: &mut Vec<AssetPath>) -> SpriteInfo {
        let path = AssetPath::new(load_context.path().parent().unwrap().join(&self.atlas), None);
        dependencies.push(path.clone());
        let texture_handle = load_context.get_handle(path);
        let texture_atlas  = TextureAtlas::from_grid(texture_handle, Vec2::new(TILE_SIZE * self.width as f32, TILE_SIZE * self.height as f32), self.columns, self.rows);
        let atlas = load_context.set_labeled_asset("atlas", LoadedAsset::new(texture_atlas));
        SpriteInfo {
            atlas,
            anim: self.anim.to_anim_info(),
            columns: self.columns,
            rows: self.rows,
            width: self.width,
            height: self.height,
            scale: self.scale,
        }
    }
}

#[derive(Clone, Debug)]
pub enum AnimInfo {
    Static {
        index: u32,
        tint: Palette,
    },
    Animated {
        def_anim_idx:  usize,
        anim_vec:      Vec<Animation>,
        name_to_index: HashMap<String, usize>,
    }
}

impl AnimInfo {
    pub fn default_tint(&self) -> Palette {
        match self {
            AnimInfo::Static {tint, ..} => tint.clone(),
            AnimInfo::Animated {def_anim_idx, anim_vec, ..} => anim_vec[def_anim_idx.clone()].frames[0].tint.clone(),
        }
    }

    pub fn default_index(&self) -> u32 {
        match self {
            AnimInfo::Static {index, ..} => index.clone(),
            AnimInfo::Animated {def_anim_idx, anim_vec, ..} => anim_vec[def_anim_idx.clone()].frames[0].index.clone(),
        }
    }

    pub fn default_anim_state(&self) -> Option<AnimState> {
        match self {
            AnimInfo::Static {..} => None,
            AnimInfo::Animated {def_anim_idx, anim_vec, ..} => Some(AnimState {
                cur_frame_idx: 0,
                cur_anim_idx: def_anim_idx.clone(),
                timer:        Timer::new(anim_vec[def_anim_idx.clone()].delay, true),
            }),
        }
    }
}

#[derive(Clone, Debug, TypeUuid)]
#[uuid = "391f8075-c5f5-49f4-af17-ca2f3bf5c74d"]
pub struct SpriteInfo {
    pub atlas:   Handle<TextureAtlas>,
    pub columns: usize,
    pub rows:    usize,
    pub width:   usize,
    pub height:  usize,
    pub scale:   Vec3,
    pub anim:    AnimInfo,
}

#[derive(Clone, Debug, TypeUuid)]
#[uuid = "baae31cb-323e-418a-b820-374ddf9c7159"]
pub struct AnimState {
    pub cur_anim_idx:  usize,
    pub cur_frame_idx: usize,
    pub timer:         Timer,
}

impl AnimState {
    pub fn update(&mut self, info: &SpriteInfo, time: &Time) {
        self.timer.tick(time.delta());
        if self.timer.finished() {
            match &info.anim {
                AnimInfo::Animated {anim_vec, ..} => {
                    self.cur_frame_idx = (self.cur_frame_idx + 1) % anim_vec[self.cur_anim_idx].frames.len();
                },
                _ => (),
            }
        }
    }

    pub fn cur_tint(&self, info: &SpriteInfo) -> Palette {
        match &info.anim {
            AnimInfo::Animated {anim_vec, ..} => anim_vec[self.cur_anim_idx].frames[self.cur_frame_idx].tint.clone(),
            _ => info.anim.default_tint(),
        }
    }

    pub fn cur_index(&self, info: &SpriteInfo) -> u32 {
        match &info.anim {
            AnimInfo::Animated {anim_vec, ..} => anim_vec[self.cur_anim_idx].frames[self.cur_frame_idx].index.clone(),
            _ => info.anim.default_index(),
        }
    }
}
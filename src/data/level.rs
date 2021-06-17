use bevy::{
    prelude::*,
    reflect::TypeUuid,
};
use serde::{Serialize, Deserialize};

use crate::data::action::*;

#[derive(Clone, Debug, Default)]
pub struct MapScale(pub f32);

#[derive(Clone, Default, TypeUuid)]
#[uuid = "277256ba-9bdd-4263-a396-b6bca19f7833"]
pub struct LevelInfo {
    pub title: String,
    pub subtitle: Option<String>,
    pub level_idx: usize,
}

#[derive(TypeUuid)]
#[uuid = "caf9e6e6-7677-49ab-94df-a3a4c354f6d7"]
pub struct LevelToLoad(pub usize);


#[derive(TypeUuid)]
#[uuid = "8bf0327e-2d5c-42ef-b614-f883ae078b0b"]
pub struct OwningLevel(pub Entity);

#[derive(Clone, Copy, Debug, Default)]
pub struct Pos {
    pub x: i32,
    pub y: i32, // +y is down, unlike in Bevy rendering
    pub z: i32,
}

impl Pos {
    pub fn step(&self, dir: Dir) -> Pos {
        match dir {
            Dir::North     => Pos { y: self.y - 1,                ..self.clone()},
            Dir::Northeast => Pos { y: self.y - 1, x: self.x + 1, ..self.clone()},
            Dir::East      => Pos {                x: self.x + 1, ..self.clone()},
            Dir::Southeast => Pos { y: self.y + 1, x: self.x + 1, ..self.clone()},
            Dir::South     => Pos { y: self.y + 1,                ..self.clone()},
            Dir::Southwest => Pos { y: self.y + 1, x: self.x - 1, ..self.clone()},
            Dir::West      => Pos {                x: self.x - 1, ..self.clone()},
            Dir::Northwest => Pos { y: self.y - 1, x: self.x - 1, ..self.clone()},
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PosState {
    None,
    Solid,
    Floorless,
    Entity(Entity),
    //Item(Handle<Todo>),
    Damaging(f32),
}

impl Default for PosState {
    fn default() -> Self { PosState::None }
}

impl PosState {
    pub fn is_blocking(&self) -> bool {
        match self {
            PosState::None | PosState::Damaging(_) => false,
            _ => true,
        }
    }
}

#[derive(Debug, TypeUuid)]
#[uuid = "2b16de55-c777-41ea-a05b-e62c4a5e1b46"]
pub struct Grid(pub Vec<Vec<Vec<PosState>>>);

impl Grid {
    pub fn new(width: usize, height: usize, layers: usize) -> Grid {
        Grid(vec![vec![vec![PosState::None; width]; height]; layers])
    }

    pub fn get(&self, pos: &Pos) -> PosState {
        if pos.x < 0 || pos.y < 0 || pos.z < 0 {
            PosState::default()
        } else {
            if let Some(layer) = self.0.get(pos.z as usize) {
                if let Some(row) = layer.get(pos.y as usize) {
                    row.get(pos.x as usize)
                        .map(|s| s.clone())
                        .unwrap_or_default()
                } else {
                    PosState::default()
                }
            } else {
                PosState::default()
            }
        }
    }

    pub fn set(&mut self, pos: &Pos, state: PosState) {
        if !(pos.x < 0 || pos.y < 0 || pos.z < 0) {
            if let Some(layer) = self.0.get_mut(pos.z as usize) {
                if let Some(row) = layer.get_mut(pos.y as usize) {
                    let len = row.len() as i32;
                    if pos.x >= 0 && pos.x < len {
                        row[pos.x as usize] = state;
                    }
                }
            }
        } else {
            println!("Attempted to set PosState in grid with negative coordinates {:?}", pos);
        }
    }
}
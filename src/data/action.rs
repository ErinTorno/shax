use bevy::prelude::*;

use crate::util::types::*;

pub const SECONDS_TO_WALK: f32 = 0.2;
pub const SECONDS_TO_RUN: f32 = 0.1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Dir {
    North, Northeast, East, Southeast, South, Southwest, West, Northwest,
}

#[derive(Clone, Debug)]
pub struct LocalActions {
    pub north:      TimeStamped<bool>,
    pub south:      TimeStamped<bool>,
    pub east:       TimeStamped<bool>,
    pub west:       TimeStamped<bool>,
    pub run:        TimeStamped<bool>,
    pub interact:   TimeStamped<bool>,
    pub move_timer: Timer,
}

impl Default for LocalActions {
    fn default() -> LocalActions {
        LocalActions {
            north:    TimeStamped::default(),
            south:    TimeStamped::default(),
            east:     TimeStamped::default(),
            west:     TimeStamped::default(),
            run:      TimeStamped::default(),
            interact: TimeStamped::default(),
            move_timer: Timer::from_seconds(SECONDS_TO_WALK, false),
        }
    }
}

impl LocalActions {
    pub fn dir(&self, seconds_elapsed: f64) -> Option<Dir> {
        let ns_comp = index_component(IDX_NORTH, IDX_SOUTH, &self.north, &self.south, seconds_elapsed);
        let ew_comp = index_component(IDX_EAST , IDX_WEST,  &self.east,  &self.west, seconds_elapsed);
        index_to_direction(ns_comp + ew_comp)
    }
}

#[derive(Clone, Debug)]
pub struct ControlSettings {
    pub north: KeyCode,
    pub south: KeyCode,
    pub east:  KeyCode,
    pub west:  KeyCode,
    pub run:   KeyCode,
}

impl Default for ControlSettings {
    fn default() -> Self {
        ControlSettings {
            north: KeyCode::W,
            south: KeyCode::S,
            east:  KeyCode::D,
            west:  KeyCode::A,
            run:   KeyCode::LShift,
        }
    }
}

// Private

const INPUT_DELAY_SECONDS: f64 = 5. / 60.; // 5 frames

fn index_component(res_a: i32, res_b: i32, a: &TimeStamped<bool>, b: &TimeStamped<bool>, seconds_elapsed: f64) -> i32 {
    // if delay hasn't occured in either input, wait a short delay to help enable diagonal movement
    if a.timestamp + INPUT_DELAY_SECONDS > seconds_elapsed && b.timestamp + INPUT_DELAY_SECONDS > seconds_elapsed {
        IDX_NONE
    } else if a.value && b.value {
        if      a.timestamp > b.timestamp { res_a }
        else if a.timestamp < b.timestamp { res_b }
        else    { IDX_NONE }
    }
    else if a.value { res_a }
    else if b.value { res_b }
    else            { IDX_NONE }
}

fn index_to_direction(idx: i32) -> Option<Dir> {
    match idx {
        IDX_NORTH     => Some(Dir::North),
        IDX_NORTHEAST => Some(Dir::Northeast),
        IDX_EAST      => Some(Dir::East),
        IDX_SOUTHEAST => Some(Dir::Southeast),
        IDX_SOUTH     => Some(Dir::South),
        IDX_SOUTHWEST => Some(Dir::Southwest),
        IDX_WEST      => Some(Dir::West),
        IDX_NORTHWEST => Some(Dir::Northwest),
        _                 => None,
    }
}

const IDX_NORTH:     i32 = 1;
const IDX_NORTHEAST: i32 = 5;
const IDX_EAST:      i32 = 4;
const IDX_SOUTHEAST: i32 = 3;
const IDX_SOUTH:     i32 = -1;
const IDX_SOUTHWEST: i32 = -5;
const IDX_WEST:      i32 = -4;
const IDX_NORTHWEST: i32 = -3;
const IDX_NONE:      i32 = 0;
use std::default::Default;

use serde::{Deserialize, Serialize};

use crate::core::{battle::movement::MovePoints, map::PosHex};

pub use crate::core::battle::{check::check, execute::execute, state::State};

pub mod ability;
pub mod ai;
pub mod command;
pub mod component;
pub mod effect;
pub mod event;
pub mod execute;
pub mod movement;
pub mod scenario;
pub mod state;
pub mod utils;

mod check;

#[cfg(test)]
mod tests;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlayerId(pub i32);

/// An index of player's turn.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Phase(i32);

impl Phase {
    pub fn from_player_id(player_id: PlayerId) -> Self {
        Phase(player_id.0 as _)
    }
}

#[derive(
    Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash,
)]
pub struct ObjId(i32);

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Strength(pub i32);

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Attacks(pub i32);

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Moves(pub i32);

/// Move or Attack
#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Jokers(pub i32);

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Accuracy(pub i32);

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Dodge(pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Eq)]
pub enum TileType {
    Plain,
    Rocks,
}

impl Default for TileType {
    fn default() -> Self {
        TileType::Plain
    }
}

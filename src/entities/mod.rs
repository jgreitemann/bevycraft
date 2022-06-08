mod avatar;
mod mobs;

use crate::prelude::*;
use avatar::*;
use bevy::app::PluginGroupBuilder;
use mobs::*;

pub use mobs::{HostileMobBundle, PlayerBundle};

pub struct EntityPlugins;

impl PluginGroup for EntityPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(MobPlugin)
            .add_after::<MobPlugin, AvatarPlugin>(AvatarPlugin);
    }
}

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Hostile;

#[derive(Component, Debug)]
pub struct Mob;

pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Position,
}

#[derive(Clone, Copy, Component, Debug, Default)]
pub struct Position(pub IVec2);

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position(IVec2::new(x, y))
    }
}

impl From<TilePos> for Position {
    fn from(tile_pos: TilePos) -> Self {
        Self(IVec2::new(tile_pos.0 as i32, tile_pos.1 as i32))
    }
}

impl From<Point> for Position {
    fn from(p: Point) -> Self {
        Self(IVec2::new(p.x, p.y))
    }
}

impl TryFrom<Position> for TilePos {
    type Error = ();

    fn try_from(Position(vec): Position) -> Result<Self, Self::Error> {
        if vec.x < 0 || vec.y < 0 {
            Err(())
        } else {
            Ok(TilePos(vec.x as u32, vec.y as u32))
        }
    }
}

impl From<Position> for Point {
    fn from(Position(vec): Position) -> Self {
        Point::new(vec.x, vec.y)
    }
}

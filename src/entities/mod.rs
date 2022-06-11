mod avatar;
mod combat;
mod mobs;

use crate::prelude::*;
use bevy::app::PluginGroupBuilder;

use avatar::*;
use combat::*;
use mobs::*;

pub use mobs::{HostileMobBundle, PlayerBundle};

pub struct EntityPlugins;

impl PluginGroup for EntityPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(MobPlugin)
            .add(CombatPlugin)
            .add_after::<MobPlugin, AvatarPlugin>(AvatarPlugin);
    }
}

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Hostile;

#[derive(Component, Debug)]
pub struct Mob;

#[derive(Component, Debug)]
pub struct Health {
    current: u32,
    max: u32,
}

impl Health {
    pub fn take_damage(&mut self, amount: u32) -> u32 {
        let damage_dealt = u32::min(self.current, amount);
        self.current -= damage_dealt;
        damage_dealt
    }

    pub fn heal(&mut self, amount: u32) -> u32 {
        let health_restored = u32::min(self.max - self.current, amount);
        self.current += health_restored;
        health_restored
    }

    pub fn is_dead(&self) -> bool {
        self.current == 0
    }

    pub fn hitpoints(&self) -> u32 {
        self.current
    }
}

pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Position,
}

pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

#[derive(Clone, Copy, Component, Debug, Default, PartialEq)]
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

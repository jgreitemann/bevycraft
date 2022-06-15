use crate::prelude::*;
use std::collections::HashSet;

pub struct FieldOfViewPlugin;

impl Plugin for FieldOfViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_fov_after_mob_movement)
            .add_system(update_fov_after_tile_type_changed)
            .add_system_to_stage(CoreStage::PostUpdate, update_tile_visibility)
            .add_system_to_stage(CoreStage::PostUpdate, update_mob_visibility);
    }
}

#[derive(Clone, Component, Debug, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
        }
    }

    pub fn can_see(&self, position: Position) -> bool {
        self.visible_tiles.contains(&position.into())
    }
}

fn update_fov_after_mob_movement(
    mut fov_query: Query<(&Position, &mut FieldOfView), (With<Mob>, Changed<Position>)>,
    tile_map_query: TileMapQuery,
) {
    let map = MapAdapter::new(tile_map_query);
    for (&pos, mut fov) in fov_query.iter_mut() {
        fov.visible_tiles = field_of_view_set(pos.into(), fov.radius, &map);
    }
}

fn update_fov_after_tile_type_changed(
    mut fov_query: Query<(&Position, &mut FieldOfView), With<Mob>>,
    tile_type_query: Query<(), Changed<TileType>>,
    tile_map_query: TileMapQuery,
) {
    if tile_type_query.iter().next().is_some() {
        let map = MapAdapter::new(tile_map_query);
        for (&pos, mut fov) in fov_query.iter_mut() {
            fov.visible_tiles = field_of_view_set(pos.into(), fov.radius, &map);
        }
    }
}

fn update_tile_visibility(
    player_fov_query: Query<&FieldOfView, (With<Player>, Changed<FieldOfView>)>,
    mut tile_query: Query<(&TilePos, &mut Tile)>,
) {
    if let Some(player_fov) = player_fov_query.iter().next() {
        for (&tile_pos, mut tile) in tile_query.iter_mut() {
            tile.color = if player_fov.can_see(tile_pos.into()) {
                Color::WHITE
            } else {
                Color::GRAY
            };
        }
    }
}

fn update_mob_visibility(
    player_fov_query: Query<&FieldOfView, (With<Player>, Changed<FieldOfView>)>,
    mut mob_query: Query<(&mut Visibility, &Position), With<Hostile>>,
) {
    if let Some(player_fov) = player_fov_query.iter().next() {
        for (mut visibility, &pos) in mob_query.iter_mut() {
            visibility.is_visible = player_fov.can_see(pos);
        }
    }
}

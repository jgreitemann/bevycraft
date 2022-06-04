use crate::prelude::*;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WantsToMove>()
            .add_startup_system(spawn_player)
            .add_system(movement)
            .add_system(avatar_tracks_mob_position.after(movement));
    }
}

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Mob;

#[derive(Clone, Copy, Component, Debug)]
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

pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Position,
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    mob: Mob,
    position: Position,
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

impl PlayerBundle {
    fn new(position: Position, texture_atlases: &Assets<TextureAtlas>) -> Self {
        let atlas_handle = texture_atlases.get_handle(texture_atlases.iter().next().unwrap().0);
        let world_pos = tile_center(&position);
        PlayerBundle {
            player: Player,
            mob: Mob,
            position,
            sprite_sheet_bundle: SpriteSheetBundle {
                transform: Transform::from_translation(world_pos),
                texture_atlas: atlas_handle,
                sprite: TextureAtlasSprite::new(64),
                ..default()
            },
        }
    }
}

fn spawn_player(mut commands: Commands, texture_atlases: Res<Assets<TextureAtlas>>) {
    commands.spawn_bundle(PlayerBundle::new(Position::new(2, 2), &texture_atlases));
}

fn movement(
    mut msgs: EventReader<WantsToMove>,
    mut commands: Commands,
    mut tile_map_query: TileMapQuery,
) {
    for WantsToMove {
        entity,
        destination,
    } in msgs.iter()
    {
        if tile_map_query.can_enter_tile(destination) {
            commands.entity(*entity).insert(*destination);
        }
    }
}

fn avatar_tracks_mob_position(mut query: Query<(&mut Transform, &Position), With<Mob>>) {
    for (mut transform, pos) in query.iter_mut() {
        transform.translation = tile_center(pos);
    }
}

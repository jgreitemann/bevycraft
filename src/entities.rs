use crate::prelude::*;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WantsToMove>()
            .add_system(movement)
            .add_system(avatar_tracks_mob_position.after(movement))
            .add_system(avatar_animation);
    }
}

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Hostile;

#[derive(Component, Debug)]
pub struct Mob;

#[derive(Component, Debug)]
pub struct AvatarAnimation {
    pub timer: Timer,
    pub destination: Vec3,
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

pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Position,
}

#[derive(Bundle)]
struct MobBundle {
    mob: Mob,
    position: Position,
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

impl MobBundle {
    pub fn new(
        position: Position,
        texture_index: usize,
        texture_atlases: &Assets<TextureAtlas>,
    ) -> Self {
        let atlas_handle = texture_atlases.get_handle(texture_atlases.iter().next().unwrap().0);
        let world_pos = tile_center(&position);
        MobBundle {
            mob: Mob,
            position,
            sprite_sheet_bundle: SpriteSheetBundle {
                transform: Transform::from_translation(world_pos),
                texture_atlas: atlas_handle,
                sprite: TextureAtlasSprite::new(texture_index),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    #[bundle]
    mob_bundle: MobBundle,
}

impl PlayerBundle {
    pub fn new(position: Position, texture_atlases: &Assets<TextureAtlas>) -> Self {
        PlayerBundle {
            player: Player,
            mob_bundle: MobBundle::new(position, 64, texture_atlases),
        }
    }
}

#[derive(Bundle)]
pub struct HostileMobBundle {
    hostile: Hostile,
    #[bundle]
    mob_bundle: MobBundle,
}

impl HostileMobBundle {
    pub fn new(position: Position, texture_atlases: &Assets<TextureAtlas>) -> Self {
        HostileMobBundle {
            hostile: Hostile,
            mob_bundle: MobBundle::new(position, 103, texture_atlases),
        }
    }
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

fn avatar_tracks_mob_position(
    mut query: Query<(Entity, &Position), (With<Mob>, Changed<Position>)>,
    mut commands: Commands,
) {
    for (entity, pos) in query.iter_mut() {
        commands.entity(entity).insert(AvatarAnimation {
            timer: Timer::from_seconds(0.1, false),
            destination: tile_center(pos),
        });
    }
}

fn avatar_animation(
    mut query: Query<(Entity, &mut AvatarAnimation, &mut Transform)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let delta_t = time.delta();
    for (entity, mut animation, mut transform) in query.iter_mut() {
        let before = animation.timer.percent_left();
        animation.timer.tick(delta_t);
        let after = animation.timer.percent_left();
        let delta_v = animation.destination - transform.translation;
        transform.translation += (before - after) / before * delta_v;

        if animation.timer.finished() {
            commands.entity(entity).remove::<AvatarAnimation>();
        }
    }
}

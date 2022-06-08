use crate::prelude::*;

pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WantsToMove>()
            .add_event::<WantsToAttack>()
            .add_system(movement);
    }
}

#[derive(Bundle)]
struct MobBundle {
    mob: Mob,
    position: Position,
    health: Health,
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
            health: Health {
                current: 10,
                max: 10,
            },
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
    mut movement_msgs: EventReader<WantsToMove>,
    mut commands: Commands,
    mut tile_map_query: TileMapQuery,
    mob_query: Query<(Entity, &Position), With<Mob>>,
    mut attack_msgs: EventWriter<WantsToAttack>,
) {
    for &WantsToMove {
        entity,
        destination,
    } in movement_msgs.iter()
    {
        if tile_map_query.can_enter_tile(&destination) {
            if let Some((mob, _)) = mob_query.iter().find(|(_, pos)| **pos == destination) {
                attack_msgs.send(WantsToAttack {
                    attacker: entity,
                    victim: mob,
                });
            } else {
                commands.entity(entity).insert(destination);
            }
        }
    }
}
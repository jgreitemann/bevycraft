use crate::prelude::*;

pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WantsToMove>()
            .add_event::<WantsToAttack>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                movement.run_not_in_state(TurnState::AwaitingInput),
            );
    }
}

#[derive(Bundle)]
struct MobBundle {
    mob: Mob,
    position: Position,
    health: Health,
    fov: FieldOfView,
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

impl MobBundle {
    pub fn new(
        position: Position,
        health: Health,
        fov_radius: i32,
        texture_index: usize,
        texture_atlas: &DefaultTextureAtlas,
    ) -> Self {
        let DefaultTextureAtlas(atlas_handle) = texture_atlas;
        let world_pos = tile_center(&position);
        MobBundle {
            mob: Mob,
            position,
            health,
            fov: FieldOfView::new(fov_radius),
            sprite_sheet_bundle: SpriteSheetBundle {
                transform: Transform::from_translation(world_pos),
                texture_atlas: atlas_handle.clone(),
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
    pub fn new(position: Position, texture_atlas: &DefaultTextureAtlas) -> Self {
        PlayerBundle {
            player: Player,
            mob_bundle: MobBundle::new(position, Health::new(10), 8, 64, texture_atlas),
        }
    }
}

#[derive(Bundle)]
pub struct HostileMobBundle {
    hostile: Hostile,
    chasing: ChasingPlayer,
    #[bundle]
    mob_bundle: MobBundle,
}

impl HostileMobBundle {
    pub fn new(position: Position, texture_atlas: &DefaultTextureAtlas) -> Self {
        HostileMobBundle {
            hostile: Hostile,
            chasing: ChasingPlayer,
            mob_bundle: MobBundle::new(position, Health::new(1), 6, 103, texture_atlas),
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

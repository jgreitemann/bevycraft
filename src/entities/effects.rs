use crate::prelude::*;
use bevy::ecs::system::EntityCommands;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_healing).add_system(reveal_map);
    }
}

#[derive(Component, Debug, PartialEq)]
struct ApplyHealing(pub u32);

#[derive(Component, Debug)]
struct RevealMap;

pub fn apply_effect_to_mob(entity_commands: &mut EntityCommands, effect: &EffectData) {
    match effect {
        EffectData::Healing(amount) => entity_commands.insert(ApplyHealing(*amount)),
        EffectData::RevealMap => entity_commands.insert(RevealMap),
    };
}

fn apply_healing(
    mut commands: Commands,
    mut mob_query: Query<(Entity, &ApplyHealing, &mut Health)>,
) {
    for (entity, &ApplyHealing(amount), mut health) in mob_query.iter_mut() {
        health.heal(amount);
        commands.entity(entity).remove::<ApplyHealing>();
    }
}

fn reveal_map(
    mut commands: Commands,
    mob_query: Query<Entity, With<RevealMap>>,
    mut tile_query: Query<&mut Tile>,
) {
    for entity in mob_query.iter() {
        tile_query
            .iter_mut()
            .for_each(|mut tile| tile.visible = true);
        commands.entity(entity).remove::<RevealMap>();
    }
}

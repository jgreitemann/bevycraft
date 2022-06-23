use crate::prelude::*;
use bevy::ecs::system::EntityCommands;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_healing);
    }
}

#[derive(Component, Debug, PartialEq)]
struct ApplyHealing(pub u32);

pub fn apply_effect_to_mob(entity_commands: &mut EntityCommands, effect: &EffectData) {
    entity_commands.insert(match effect {
        EffectData::Healing(amount) => ApplyHealing(*amount),
    });
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

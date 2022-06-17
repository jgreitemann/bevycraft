use crate::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(combat_damage)
            .add_system_to_stage(CoreStage::Last, kill_mobs);
    }
}

fn combat_damage(
    mut attack_msgs: EventReader<WantsToAttack>,
    mut health_query: Query<&mut Health>,
) {
    for &WantsToAttack { victim, .. } in attack_msgs.iter() {
        if let Ok(mut victim_health) = health_query.get_mut(victim) {
            victim_health.take_damage(1);
        }
    }
}

fn kill_mobs(
    mut cmd: Commands,
    health_query: Query<(Entity, &Health, Option<&Player>), Changed<Health>>,
) {
    for (killed_entity, _, player_opt) in health_query
        .iter()
        .filter(|(_, health, _)| health.is_dead())
    {
        if player_opt.is_some() {
            cmd.insert_resource(NextState(TurnState::Defeat));
        } else {
            cmd.entity(killed_entity).despawn();
        }
    }
}

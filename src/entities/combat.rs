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

fn kill_mobs(mut cmd: Commands, health_query: Query<(Entity, &Health), Changed<Health>>) {
    for (killed_entity, _) in health_query.iter().filter(|(_, health)| health.is_dead()) {
        cmd.entity(killed_entity).despawn();
    }
}

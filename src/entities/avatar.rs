use crate::prelude::*;

pub struct AvatarPlugin;

impl Plugin for AvatarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PostUpdate, avatar_tracks_mob_position)
            .add_system(avatar_animation);
    }
}

#[derive(Component, Debug)]
struct AvatarAnimation {
    pub timer: Timer,
    pub destination: Vec3,
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

        if animation.timer.finished() {
            transform.translation = animation.destination;
            commands.entity(entity).remove::<AvatarAnimation>();
        } else {
            let after = animation.timer.percent_left();
            let delta_v = animation.destination - transform.translation;
            transform.translation += (before - after) / before * delta_v;
        }
    }
}

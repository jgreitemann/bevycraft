use crate::prelude::*;
use bevy::{input::Input, render::camera::Camera};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(zooming)
            .add_system_to_stage(CoreStage::PostUpdate, track_player_avatar);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn zooming(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    for mut ortho in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Z) {
            ortho.scale += 0.1;
        }

        if keyboard_input.pressed(KeyCode::X) {
            ortho.scale -= 0.1;
        }

        if ortho.scale < 0.5 {
            ortho.scale = 0.5;
        }
    }
}

fn track_player_avatar(
    player_query: Query<&Transform, (With<Player>, Changed<Transform>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Some(new_player_pos) = player_query
        .iter()
        .next()
        .map(|transform| transform.translation.clone())
    {
        for mut camera_transform in camera_query.iter_mut() {
            let z = camera_transform.translation.z;
            camera_transform.translation = new_player_pos;
            camera_transform.translation.z = z;
        }
    }
}

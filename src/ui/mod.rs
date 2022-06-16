mod hud;

use hud::HudPlugin;

use crate::prelude::*;
use bevy::app::PluginGroupBuilder;

pub struct UiPlugins;

impl PluginGroup for UiPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(UiBasePlugin).add(HudPlugin);
    }
}

struct UiBasePlugin;

impl Plugin for UiBasePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_ui_camera);
    }
}

fn setup_ui_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

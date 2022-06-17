mod camera;
mod entities;
mod map;
mod player_input;
mod texture;
mod turn_state;
mod ui;

mod prelude {
    pub use crate::camera::*;
    pub use crate::entities::*;
    pub use crate::map::*;
    pub use crate::player_input::*;
    pub use crate::texture::*;
    pub use crate::turn_state::*;
    pub use crate::ui::*;
    pub use bevy::math::ivec2;
    pub use bevy::prelude::*;
    pub use bevy_ecs_tilemap::prelude::*;
    pub use bracket_algorithm_traits::prelude::*;
    pub use bracket_geometry::prelude::*;
    pub use bracket_pathfinding::prelude::*;
    pub use iyes_loopless::prelude::*;
}

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_ui_navigation::DefaultNavigationPlugins;
use prelude::*;

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("Bevycraft"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultNavigationPlugins)
        .add_plugin(TurnStatePlugin)
        .add_plugins(EntityPlugins)
        .add_plugins(UiPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PlayerInputPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(CameraPlugin)
        .add_startup_system_to_stage(StartupStage::PreStartup, build_texture_atlases)
        .run();
}

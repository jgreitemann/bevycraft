mod camera;
mod entities;
mod map;
mod player_input;

mod prelude {
    pub use crate::camera::*;
    pub use crate::entities::*;
    pub use crate::map::*;
    pub use bevy::math::ivec2;
    pub use bevy::prelude::*;
    pub use bevy_ecs_tilemap::prelude::*;
    pub use bracket_algorithm_traits::prelude::*;
    pub use bracket_geometry::prelude::*;
    pub use bracket_pathfinding::prelude::*;
}

use prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("Bevycraft"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(MapPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(EntityPlugin)
        .add_system_to_stage(CoreStage::PreUpdate, player_input::player_input)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

mod camera;
mod map;
mod player;

mod prelude {
    pub use crate::camera::*;
    pub use crate::map::*;
    pub use crate::player::*;
    pub use bevy::prelude::*;
    pub use bevy_ecs_tilemap::prelude::*;
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
        .add_plugin(PlayerPlugin)
        .run();
}

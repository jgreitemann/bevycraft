mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_ecs_tilemap::prelude::*;
}

use prelude::*;

mod camera;
mod map;
mod systems;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("Bevycraft"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_startup_system(camera::setup)
        .add_startup_system(map::spawn_map_layer)
        .add_plugin(systems::BevycraftPlugin)
        .run();
}

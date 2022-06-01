mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_ecs_tilemap::prelude::*;
}

use prelude::*;

mod camera;
mod map;
mod spawner;
mod systems;

pub fn build_texture_atlases(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let sprite_sheet = asset_server.load("dungeonfont.png");
    let atlas = TextureAtlas::from_grid(sprite_sheet, Vec2::new(32., 32.), 16, 16);
    texture_atlases.add(atlas);
}

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
        .add_startup_system_to_stage(StartupStage::PreStartup, build_texture_atlases)
        .add_startup_system(camera::setup)
        .add_startup_system(map::spawn_map_layer)
        .add_startup_system(spawner::spawn_player)
        .add_plugin(systems::BevycraftPlugin)
        .run();
}

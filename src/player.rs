use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn);
    }
}

fn spawn(mut commands: Commands, texture_atlases: Res<Assets<TextureAtlas>>) {
    let atlas_handle = texture_atlases.get_handle(texture_atlases.iter().next().unwrap().0);

    commands.spawn_bundle(SpriteSheetBundle {
        transform: Transform::from_xyz(0., 0., 1.),
        texture_atlas: atlas_handle,
        sprite: TextureAtlasSprite::new(64),
        ..default()
    });
}

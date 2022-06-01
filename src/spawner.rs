use crate::prelude::*;

pub fn spawn_player(mut commands: Commands, texture_atlases: Res<Assets<TextureAtlas>>) {
    let atlas_handle = texture_atlases.get_handle(texture_atlases.iter().next().unwrap().0);

    commands.spawn_bundle(SpriteSheetBundle {
        transform: Transform::from_xyz(0., 0., 1.),
        texture_atlas: atlas_handle,
        sprite: TextureAtlasSprite::new(64),
        ..default()
    });
}

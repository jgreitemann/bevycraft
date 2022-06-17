use bevy::{prelude::*, render::render_resource::TextureUsages};

#[derive(Clone, Debug)]
pub struct DefaultTextureAtlas(pub Handle<TextureAtlas>);

pub fn build_texture_atlases(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let sprite_sheet = asset_server.load("dungeonfont.png");
    let atlas = TextureAtlas::from_grid(sprite_sheet, Vec2::new(32., 32.), 16, 16);
    commands.insert_resource(DefaultTextureAtlas(texture_atlases.add(atlas)));
}

#[allow(dead_code)]
pub fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(mut texture) = textures.get_mut(handle) {
                    texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_SRC
                        | TextureUsages::COPY_DST;
                }
            }
            _ => (),
        }
    }
}

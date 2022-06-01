mod movement;
mod texture;

use crate::prelude::*;

pub struct BevycraftPlugin;

impl Plugin for BevycraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(movement::movement)
            .add_system(texture::set_texture_filters_to_nearest);
    }
}

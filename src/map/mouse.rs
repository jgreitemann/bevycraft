use crate::prelude::*;
use bevy::input::mouse::MouseButtonInput;

pub struct TileInteraction(TilePos);

pub fn mouse_click_tile_interaction(
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    mut tile_evw: EventWriter<TileInteraction>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    use bevy::input::ElementState;

    let (camera, camera_transform) = q_camera.iter().next().unwrap();

    for ev in mousebtn_evr.iter() {
        match ev.state {
            ElementState::Released => {
                let window = windows.get_primary().unwrap();
                if let Some(screen_pos) = window.cursor_position() {
                    // get the size of the window
                    let window_size = Vec2::new(window.width() as f32, window.height() as f32);

                    // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
                    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

                    // matrix for undoing the projection and camera transform
                    let ndc_to_world =
                        camera_transform.compute_matrix() * camera.projection_matrix.inverse();

                    // use it to convert ndc to world-space coordinates
                    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

                    // reduce it to a 2D value
                    let world_pos: Vec2 = world_pos.truncate();
                    tile_evw.send(TileInteraction(super::world_to_tile(&world_pos)));
                }
            }
            _ => {}
        }
    }
}

pub fn hide_tiles_by_click(
    mut tile_evr: EventReader<TileInteraction>,
    mut tile_query: Query<(&TilePos, &mut Tile)>,
    mut map_query: MapQuery,
) {
    for TileInteraction(tile_pos) in tile_evr.iter() {
        if let Some((_, mut tile)) = tile_query.iter_mut().find(|(pos, _)| *pos == tile_pos) {
            tile.visible = false;
            map_query.notify_chunk_for_tile(*tile_pos, MAP_ID, MAP_LAYER_ID);
        }
    }
}

use crate::prelude::*;
use bevy::input::{keyboard::KeyboardInput, ElementState};

pub fn player_input(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut msgs: EventWriter<WantsToMove>,
    player_query: Query<(Entity, &Position), With<Player>>,
) {
    let (player_entity, player_pos) = player_query.single();

    for event in keyboard_input_events.iter() {
        if let KeyboardInput {
            key_code: Some(key),
            state: ElementState::Pressed,
            ..
        } = event
        {
            match key {
                KeyCode::A => msgs.send(move_player(player_entity, player_pos, ivec2(-1, 0))),
                KeyCode::D => msgs.send(move_player(player_entity, player_pos, ivec2(1, 0))),
                KeyCode::W => msgs.send(move_player(player_entity, player_pos, ivec2(0, 1))),
                KeyCode::S => msgs.send(move_player(player_entity, player_pos, ivec2(0, -1))),
                _ => {}
            }
        }
    }
}

fn move_player(
    player_entity: Entity,
    &Position(player_vec): &Position,
    delta: IVec2,
) -> WantsToMove {
    let destination = Position(player_vec + delta);
    WantsToMove {
        entity: player_entity,
        destination,
    }
}

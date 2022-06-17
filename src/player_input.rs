use crate::prelude::*;
use bevy::input::{keyboard::KeyboardInput, ElementState};

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            CoreStage::PreUpdate,
            player_input.run_in_state(TurnState::AwaitingInput),
        );
    }
}

fn player_input(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut msgs: EventWriter<WantsToMove>,
    player_query: Query<(Entity, &Position), With<Player>>,
    mut commands: Commands,
) {
    let (player_entity, &Position(player_vec)) = player_query.single();

    for event in keyboard_input_events.iter() {
        if let KeyboardInput {
            key_code: Some(key),
            state: ElementState::Pressed,
            ..
        } = event
        {
            use KeyCode::*;
            let next_state = match key {
                W | A | S | D => {
                    let delta = match key {
                        W => ivec2(0, 1),
                        A => ivec2(-1, 0),
                        S => ivec2(0, -1),
                        D => ivec2(1, 0),
                        _ => unreachable!(),
                    };
                    msgs.send(WantsToMove {
                        entity: player_entity,
                        destination: Position(player_vec + delta),
                    });
                    NextState(TurnState::PlayerTurn)
                }
                Space => NextState(TurnState::PlayerTurn),
                Escape => NextState(TurnState::Pause),
                _ => continue,
            };

            commands.insert_resource(next_state);
        }
    }
}

use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    Victory,
    Defeat,
    Pause,
    Reset,
}

pub struct TurnStatePlugin;

impl Plugin for TurnStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(TurnState::AwaitingInput)
            .add_system_to_stage(CoreStage::Last, end_turn);
    }
}

fn end_turn(turn_state: Res<CurrentState<TurnState>>, mut commands: Commands) {
    use TurnState::*;
    let CurrentState(current_state) = turn_state.as_ref();
    commands.insert_resource(NextState(match current_state {
        AwaitingInput | Victory | Defeat | Pause => return,
        PlayerTurn => MonsterTurn,
        MonsterTurn => AwaitingInput,
        Reset => AwaitingInput,
    }));
}

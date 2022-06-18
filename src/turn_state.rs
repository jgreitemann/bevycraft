use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TurnState {
    Loading,
    NewGame,
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    Victory,
    Defeat,
    Pause,
}

pub struct TurnStatePlugin;

impl Plugin for TurnStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(TurnState::Loading)
            .add_system_to_stage(CoreStage::Last, end_turn);
    }
}

fn end_turn(
    turn_state: Res<CurrentState<TurnState>>,
    mut commands: Commands,
    amulet_query: Query<Entity, (With<AmuletOfYala>, With<CarriedBy>)>,
) {
    use TurnState::*;

    let next_state = if let Some(amulet_entity) = amulet_query.iter().next() {
        commands.entity(amulet_entity).despawn();
        Victory
    } else {
        let CurrentState(current_state) = turn_state.as_ref();
        match current_state {
            Loading | AwaitingInput | Victory | Defeat | Pause => return,
            PlayerTurn => MonsterTurn,
            MonsterTurn => AwaitingInput,
            NewGame => AwaitingInput,
        }
    };

    commands.insert_resource(NextState(next_state));
}

use bevy::prelude::*;

use super::SimulationState;



/*
    ----------------------
    ---- Toggle Pause ----
    ----------------------
*/

pub fn toggle_pause_game(
    mut next_state: ResMut<NextState<SimulationState>>,
    state: Res<State<SimulationState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        match state.get() {
            SimulationState::Paused => {next_state.set(SimulationState::Running); println!("SimulationState: SimulationState::Running")},
            SimulationState::Running => {next_state.set(SimulationState::Paused); println!("SimulationState: SimulationState::Paused")},
        }
    }
}
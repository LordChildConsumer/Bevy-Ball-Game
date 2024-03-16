use bevy::{
    prelude::*,
    window::PrimaryWindow,
    app::AppExit,
};

use super::SimulationState;




/*
    ----------------------
    ---- Spawn Camera ----
    ----------------------
*/

pub fn spawn_camera(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_q.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 10.0),
            ..default()
        }
    );
}




/*
    ----------------------------
    ---- Quit Game With Esc ----
    ----------------------------
*/

pub fn quit_game(
    mut app_exit_ew: EventWriter<AppExit>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        app_exit_ew.send(AppExit);
    }
}



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




/*
    --------------------------
    ---- Pause Simulation ----
    --------------------------
*/

pub fn pause_simulation(
    mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
    next_sim_state.set(SimulationState::Paused);
}




/*
    ----------------------------
    ---- Unpause Simulation ----
    ----------------------------
*/

pub fn unpause_simulation(
    mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
    next_sim_state.set(SimulationState::Running);
}
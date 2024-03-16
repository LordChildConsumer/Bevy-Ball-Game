use bevy::{
    prelude::*,
    window::Window,
    math::Vec3,
};

use crate::game::SimulationState;
use crate::AppState;




/*
    ---------------------
    ---- Transitions ----
    ---------------------
*/

// How to use match for this:
// https://bevy-cheatbook.github.io/input/keyboard.html#keyboard-events
pub fn transition_to_game_state(
    mut next_state: ResMut<NextState<AppState>>,
    keys: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keys.just_pressed(KeyCode::KeyG) {
        if app_state.get() != &AppState::Game {
            next_state.set(AppState::Game);
            println!("Entered: AppState::Game");
        }
    }
}

pub fn transition_to_main_menu_state(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
    keys: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keys.just_pressed(KeyCode::KeyM) {
        if app_state.get() != &AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
            next_sim_state.set(SimulationState::Paused);
            println!("AppState: AppState::MainMenu");
        }
    }
}




/*
    ----------------------------------
    ---- Clamp Position To Window ----
    ----------------------------------
*/

pub fn clamp_to_window(x: f32, y: f32, radius: f32, window: &Window) -> Vec3 {
    let x_min = 0.0 + radius;
    let y_min = 0.0 + radius;
    let x_max = window.width() - radius;
    let y_max = window.height() - radius;

    return Vec3::new(
        x.clamp(x_min, x_max),
        y.clamp(y_min, y_max),
        0.0
    );
}


pub fn clamp_to_window_margin(x: f32, y: f32, radius: f32, margin: f32, window: &Window) -> Vec3 {
    return clamp_to_window(x, y, radius + margin, window);
}
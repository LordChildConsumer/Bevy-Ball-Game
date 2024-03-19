use bevy::{
    prelude::*,
    app::AppExit,
};

use crate::AppState;
use crate::game::ui::game_over::{
    components::*,
    styles::*,
};




/*
    ------------------------
    ---- Restart Button ----
    ------------------------
*/

pub fn interact_with_restart_button(
    mut button_q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<RestartButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color) in button_q.iter_mut() {
        match *interaction {
            // Pressed
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_app_state.set(AppState::Game);
            }

            // Hovered
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }

            // None
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}




/*
    --------------------------
    ---- Main Menu Button ----
    --------------------------
*/

pub fn interact_with_menu_button(
    mut button_q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<MainMenuButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color) in button_q.iter_mut() {
        match *interaction {
            // Pressed
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_app_state.set(AppState::MainMenu);
            }

            // Hovered
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }

            // None
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}




/*
    ---------------------
    ---- Quit Button ----
    ---------------------
*/

pub fn interact_with_quit_button(
    mut button_q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<QuitButton>)>,
    mut app_exit_ew: EventWriter<AppExit>,
) {
    for (interaction, mut color) in button_q.iter_mut() {
        match *interaction {
            // Pressed
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                app_exit_ew.send(AppExit);
            }

            // Hovered
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }

            // None
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
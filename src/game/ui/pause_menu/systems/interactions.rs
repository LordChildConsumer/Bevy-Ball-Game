use bevy::{
    app::AppExit,
    prelude::*,
};

use crate::AppState;
use crate::game::{
    SimulationState,
    ui::pause_menu::{
        components::*,
        styles::*,
    },
};




/* 
    -----------------------
    ---- Resume Button ----
    -----------------------
*/

pub fn interact_with_resume_button(
    mut button_q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<ResumeButton>)>,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
    for (interaction, mut color) in button_q.iter_mut() {
        match *interaction {

            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_sim_state.set(SimulationState::Running);
            }

            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }

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

            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_app_state.set(AppState::MainMenu);
            }

            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }

            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }

        }
    }
}




/* 
    ----------------------------
    ---- Quit Button Button ----
    ----------------------------
*/

pub fn interact_with_quit_button(
    mut button_q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<QuitButton>)>,
    mut app_exit_ew: EventWriter<AppExit>,
) {
    for (interaction, mut color) in button_q.iter_mut() {
        match *interaction {

            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                app_exit_ew.send(AppExit);
            }

            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }

            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }

        }
    }
}
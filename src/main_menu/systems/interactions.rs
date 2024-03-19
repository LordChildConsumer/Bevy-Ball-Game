use bevy::{prelude::*, app::AppExit};
use crate::{
    AppState,
    main_menu::{
        components::{PlayButton, QuitButton},
        styles::{NORMAL_BUTTON_COLOR, HOVERED_BUTTON_COLOR, PRESSED_BUTTON_COLOR},
    },
};




/*
    ---------------------
    ---- Play Button ----
    ---------------------
*/

pub fn interact_with_play_button(
    mut button_q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<PlayButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_q.get_single_mut() {
        match *interaction {
            // Pressed
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_app_state.set(AppState::Game);
            },

            // Hover
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            },
            
            // None
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            },
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
    if let Ok((interaction, mut background_color)) = button_q.get_single_mut() {
        match *interaction {
            // Pressed
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_ew.send(AppExit);
            },

            // Hover
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            },
            
            // None
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            },
        }
    }
}
/*
    ############################
    ##                        ##
    ##        GAME HUD        ##
    ##                        ##
    ############################
*/

use bevy::prelude::*;

use crate::AppState;


mod components;
mod systems;
mod styles;


pub struct GameHudPlugin;

impl Plugin for GameHudPlugin {
    fn build(&self, app: &mut App) {
        // On Enter
        app.add_systems(OnEnter(AppState::Game), systems::layout::spawn_game_hud);

        // Update
        app.add_systems(Update, 
            (
                systems::updates::update_score_text,
                systems::updates::update_enemy_text,
            ).run_if(in_state(AppState::Game))
        );

        // On Exit
        app.add_systems(OnExit(AppState::Game), systems::layout::despawn_game_hud);
    }
}
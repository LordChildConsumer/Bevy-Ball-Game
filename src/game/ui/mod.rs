/*
    ###########################
    ##                       ##
    ##        GAME UI        ##
    ##                       ##
    ###########################
*/

use bevy::prelude::*;

mod game_over;
mod hud;
mod pause_menu;


pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            game_over::GameOverPlugin,
            hud::GameHudPlugin,
            pause_menu::PauseMenuPlugin,
        ));
    }
}
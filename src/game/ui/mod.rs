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


pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(hud::GameHudPlugin);
    }
}
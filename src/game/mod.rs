/*
    ########################
    ##                    ##
    ##        GAME        ##
    ##                    ##
    ########################
*/



pub mod enemy;
pub mod score;
pub mod star;
mod player;


use bevy::prelude::*;




/*
    ---------------------
    ---- Entry Point ----
    ---------------------
*/

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {

        // Plugins
        app.add_plugins((
            enemy::EnemyPlugin,
            score::ScorePlugin,
            star::StarPlugin,
            player::PlayerPlugin,
        ));

    }
}
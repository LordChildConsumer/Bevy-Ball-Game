use bevy::prelude::*;
use game::GamePlugin;

pub mod events;
pub mod utils;

mod main_menu;
mod game;




/*
    -------------------
    ---- App State ----
    -------------------
*/

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}




/*
    ---------------------
    ---- Entry Point ----
    ---------------------
*/

fn main() {

    let mut app: App = App::new();

    // States
    app.init_state::<AppState>();

    // Events
    app.add_event::<events::GameOver>();

    // Plugins
    app.add_plugins((
        DefaultPlugins,
        GamePlugin,
    ));

    // Systems
    app.add_systems(Update, (
        utils::transition_to_game_state,
        utils::transition_to_main_menu_state,
    ));

    app.run();

}



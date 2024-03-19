use bevy::prelude::*;
use game::GamePlugin;
use main_menu::MainManuPlugin;

pub mod events;
pub mod utils;

mod main_menu;
mod game;




/*
    -------------------
    ---- App State ----
    -------------------
*/

// FIXME: If MainMenu is default. player/enemies don't appear..?
#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum AppState {
    MainMenu,
    #[default]
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
        MainManuPlugin,
    ));

    // Systems
    app.add_systems(Update, (
        utils::transition_to_game_state,
        utils::transition_to_main_menu_state,
    ));

    app.run();

}



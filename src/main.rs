use bevy::prelude::*;
use game::GamePlugin;

pub mod events;
pub mod utils;

mod main_menu;
mod game;



fn main() {

    let mut app: App = App::new();

    app.add_event::<events::GameOver>();

    app.add_plugins((
        DefaultPlugins,
        GamePlugin,
    ));

    app.run();

}



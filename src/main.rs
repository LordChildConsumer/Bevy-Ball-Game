use bevy::prelude::*;


pub mod events;

mod enemy;
mod player;
mod game;
mod star;


use game::GamePlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use star::StarPlugin;



fn main() {

    let mut app: App = App::new();

    app.add_event::<events::GameOver>();

    app.add_plugins((
        DefaultPlugins,
        GamePlugin,
        EnemyPlugin,
        PlayerPlugin,
        StarPlugin,
    ));

    app.run();

}



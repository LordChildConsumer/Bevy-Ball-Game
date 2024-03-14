use bevy::prelude::*;

mod player;

fn main() {

    let mut app: App = App::new();

    app.add_plugins((
        DefaultPlugins,
        player::PlayerPlugin,
    ));

    app.run();

}
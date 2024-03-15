use bevy::prelude::*;

mod game;
mod camera;
mod player;
mod enemy;
mod star;

fn main() {

    let mut app: App = App::new();

    app.add_plugins((
        DefaultPlugins,
        game::GamePlugin,
        camera::CameraPlugin,
        player::PlayerPlugin,
        enemy::EnemyPlugin,
        star::StarPlugin,
    ));

    app.run();

}



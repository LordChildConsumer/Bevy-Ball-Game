use bevy::prelude::*;

mod camera;
mod player;
mod enemy;

fn main() {

    let mut app: App = App::new();

    app.add_plugins((
        DefaultPlugins,
        camera::CameraPlugin,
        player::PlayerPlugin,
        enemy::EnemyPlugin,
    ));

    app.run();

}
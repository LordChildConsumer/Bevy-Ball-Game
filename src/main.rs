use bevy::prelude::*;

mod camera;
mod player;

fn main() {

    let mut app: App = App::new();

    app.add_plugins((
        DefaultPlugins,
        camera::CameraPlugin,
        player::PlayerPlugin,
    ));

    app.run();

}
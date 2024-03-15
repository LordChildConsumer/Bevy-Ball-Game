use bevy::{
    prelude::*,
    app::AppExit,
};

mod camera;
mod player;
mod enemy;
mod star;

fn main() {

    let mut app: App = App::new();

    app.add_plugins((
        DefaultPlugins,
        camera::CameraPlugin,
        player::PlayerPlugin,
        enemy::EnemyPlugin,
        star::StarPlugin,
    ));

    app.add_systems(Update, quit_game);

    app.run();

}


/*
    ----------------------------
    ---- Quit Game With Esc ----
    ----------------------------
*/

fn quit_game(
    keys: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<AppExit>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        event_writer.send(AppExit);
    }
}
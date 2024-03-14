use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            spawn_player,
            spawn_camera,
        ));
    }
}


#[derive(Component)]
struct Player;


fn spawn_player(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_q.get_single().unwrap();
    println!("Window Size: {}/{} ", window.width(), window.height());

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player,
    ));
}


fn spawn_camera(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_q.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 10.0),
            ..default()
        }
    );
}
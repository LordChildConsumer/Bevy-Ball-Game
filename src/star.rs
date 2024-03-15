use rand::prelude::*;
use bevy::{
    prelude::*,
    window::PrimaryWindow,
};


const STAR_COUNT: i32 = 10;
const STAR_RADIUS: f32 = 15.0;




/*
    ---------------------
    ---- Entry Point ----
    ---------------------
*/

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_stars);
    }
}




/*
    --------------------
    ---- Components ----
    --------------------
*/

#[derive(Component)]
struct Star;




/*
    -----------------------------
    ---- Spawn Initial Stars ----
    -----------------------------
*/

fn spawn_stars(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_q.get_single().unwrap();

    for _ in 0..STAR_COUNT {

        let pos = clamp_to_window(
            random::<f32>() * window.width(),
            random::<f32>() * window.height(),
            window
        );


        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star,
        ));

    }
}




/*
    -------------------------
    ---- Clamp To Window ----
    -------------------------
*/

fn clamp_to_window(x: f32, y: f32, window: &Window) -> Vec3 {
    let x_min = 0.0 + STAR_RADIUS;
    let x_max = window.width() - STAR_RADIUS;
    let y_min = 0.0 + STAR_RADIUS;
    let y_max = window.height() - STAR_RADIUS;


    return Vec3::new(
        x.clamp(x_min, x_max),
        y.clamp(y_min, y_max),
        0.0
    );
}
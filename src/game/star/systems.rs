use rand::prelude::*;

use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

use super::{
    components::*,
    resources::*,

    // Constants
    INITIAL_COUNT, RADIUS,
};

use crate::{assets::StarSprite, utils::clamp_to_window};

// use super::{INITIAL_COUNT, RADIUS};

// use crate::{
//     star::components::*,
//     star::resources::*,
//     game::resources::*,
//     utils::clamp_to_window,
// };




/*
    -----------------------------
    ---- Spawn Initial Stars ----
    -----------------------------
*/

pub fn spawn_stars(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    // asset_server: Res<AssetServer>,
    star_sprite: Res<StarSprite>,
) {
    let window = window_q.get_single().unwrap();

    for _ in 0..INITIAL_COUNT {

        let pos = clamp_to_window(
            random::<f32>() * window.width(),
            random::<f32>() * window.height(),
            RADIUS,
            window,
        );

        let sprite = star_sprite.0.clone();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                texture: sprite, //asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));

    }
}




/*
    -----------------------
    ---- Despawn Stars ----
    -----------------------
*/

pub fn despawn_stars(
    mut commands: Commands,
    star_q: Query<Entity, With<Star>>,
) {
    for s_entity in star_q.iter() {
        commands.entity(s_entity).despawn();
    }
}




/*
    --------------------------
    ---- Tick Spawn Timer ----
    --------------------------
*/

pub fn tick_spawn_timer(
    mut spawn_timer: ResMut<StarSpawnTimer>,
    time: Res<Time>,
) {
    spawn_timer.timer.tick(time.delta());
}




/*
    -------------------------
    ---- Spawn Over Time ----
    -------------------------
*/

pub fn spawn_over_time(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    // asset_server: Res<AssetServer>,
    star_sprite: Res<StarSprite>,
    spawn_timer: Res<StarSpawnTimer>,
) {
    if spawn_timer.timer.finished() {
        let window = window_q.get_single().unwrap();

        // Get position
        let pos = clamp_to_window(
            random::<f32>() * window.width(),
            random::<f32>() * window.height(),
            RADIUS,
            window
        );

        let sprite = star_sprite.0.clone();
        // Spawn Star
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                texture: sprite, //asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}
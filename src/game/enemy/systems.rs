
use super::{
    components::*,
    resources::*,

    // Constants
    SPEED, RADIUS, INITIAL_COUNT, Z_INDEX, SPAWN_MARGIN,
};

use crate::utils::clamp_to_window_margin;

use bevy::{
    prelude::*,
    window::PrimaryWindow,
    audio::Volume,
};

use rand::prelude::*;




/*
    -----------------------
    ---- Spawn Enemies ----
    -----------------------
*/

pub fn spawn_enemies(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_q.get_single().unwrap();

    for _ in 0..INITIAL_COUNT {

        let pos = clamp_to_window_margin(
            random::<f32>() * window.width(),
            random::<f32>() * window.height(),
            RADIUS,
            SPAWN_MARGIN,
            window,
        );

        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(pos.x, pos.y, Z_INDEX),
                    texture: asset_server.load("sprites/ball_red_large.png"),
                    ..default()
                },
                Enemy {
                    direction: Vec2::new(random::<f32>(), random::<f32>()).normalize()
                },
            )
        );
    }
}




/*
    -------------------------
    ---- Despawn Enemies ----
    -------------------------
*/

pub fn despawn_enemies(
    mut commands: Commands,
    enemy_q: Query<Entity, With<Enemy>>,
) {
    for e_entity in enemy_q.iter() {
        commands.entity(e_entity).despawn();
    }
}



/*
    ----------------------
    ---- Move Enemies ----
    ----------------------
*/

pub fn move_enemies(
    mut enemy_q: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>
) {
    for (mut transform, enemy) in enemy_q.iter_mut() {
        
        let direction: Vec3 = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);

        // TODO: Maybe make this use clamp_to_window()
        transform.translation += direction * SPEED * time.delta_seconds();

    }
}




/*
    ---------------------------------
    ---- Update Enemy Directions ----
    ---------------------------------
*/

pub fn update_enemy_direction(
    mut commands: Commands,
    mut enemy_q: Query<(&Transform, &mut Enemy)>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {

    let window = window_q.get_single().unwrap();
    
    for (transform, mut enemy) in enemy_q.iter_mut() {
        let mut dir_changed: bool = false;
        let translation = transform.translation;
        
        // Bounce off walls
        if translation.x != translation.x.clamp(RADIUS, window.width() - RADIUS) { enemy.direction.x *= -1.0; dir_changed = true; }
        if translation.y != translation.y.clamp(RADIUS, window.height() - RADIUS) { enemy.direction.y *= -1.0; dir_changed = true; }

        if dir_changed {
            // Pick sound
            let sound: Handle<AudioSource> = if random::<f32>() > 0.5 {
                asset_server.load("sounds/enemy_bounce_1.ogg")
            } else {
                asset_server.load("sounds/enemy_bounce_2.ogg")
            };

            commands.spawn(
                AudioSourceBundle {
                    source: sound,
                    settings: PlaybackSettings {
                        volume: Volume::new(0.2),
                        ..default()
                    }
                }
            );
        }

    }
}




/*
    --------------------------
    ---- Tick Spawn Timer ----
    --------------------------
*/

pub fn tick_spawn_timer(
    mut spawn_timer: ResMut<EnemySpawnTimer>,
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
    asset_server: Res<AssetServer>,
    spawn_timer: Res<EnemySpawnTimer>,
) {
    if spawn_timer.timer.finished() {
        let window = window_q.get_single().unwrap();

        // Get position
        let pos = clamp_to_window_margin(
            random::<f32>() * window.width(),
            random::<f32>() * window.height(),
            RADIUS,
            SPAWN_MARGIN,
            window,
        );

        // Spawn Enemy
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(pos.x, pos.y, Z_INDEX),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize()
            },
        ));
    }
}
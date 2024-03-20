
use super::{
    components::*,
    resources::*,

    // Constants
    SPEED, RADIUS, INITIAL_COUNT, Z_INDEX, SPAWN_MARGIN,
};

use crate::{
    game::player::components::Player,
    assets::{EnemyBounceSound1, EnemyBounceSound2, EnemySprite},
    utils::clamp_to_window_margin,
};

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
    // asset_server: Res<AssetServer>,
    enemy_sprite: Res<EnemySprite>,
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


        let sprite = enemy_sprite.0.clone();
        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(pos.x, pos.y, Z_INDEX),
                    texture: sprite, //asset_server.load("sprites/ball_red_large.png"),
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
    mut enemy_q: Query<(&mut Transform, &mut Enemy)>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    // asset_server: Res<AssetServer>,
    bounce_sound_1: Res<EnemyBounceSound1>,
    bounce_sound_2: Res<EnemyBounceSound2>,
) {

    let window = window_q.get_single().unwrap();
    
    for (mut transform, mut enemy) in enemy_q.iter_mut() {
        let mut dir_changed: bool = false;
        let translation = transform.translation;
        
        // Bounce off walls
        if translation.x != translation.x.clamp(RADIUS, window.width() - RADIUS) { enemy.direction.x *= -1.0; dir_changed = true; }
        if translation.y != translation.y.clamp(RADIUS, window.height() - RADIUS) { enemy.direction.y *= -1.0; dir_changed = true; }

        if dir_changed {
            // Prevent a million bounces
            let clamped_pos = clamp_to_window_margin(translation.x, translation.y, RADIUS, 4.0,window);
            transform.translation.x = clamped_pos.x;
            transform.translation.y = clamped_pos.y;

            // Pick sound
            let sound: Handle<AudioSource> = if random::<f32>() > 0.5 {
                // asset_server.load("sounds/enemy_bounce_1.ogg")
                bounce_sound_1.0.clone()
            } else {
                bounce_sound_2.0.clone()
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
    player_q: Query<&Transform, With<Player>>,
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

        if let Ok(player_transform) = player_q.get_single() {

            // Prevent enemies from spawning on top of the player
            let distance = player_transform.translation.distance(pos);
            if distance > RADIUS + crate::game::player::RADIUS + SPAWN_MARGIN {

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

        
    }
}
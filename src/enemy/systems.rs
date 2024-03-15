
use super::{
    components::*,
    resources::*,

    // Constants
    SPEED, RADIUS, INITIAL_COUNT, Z_INDEX, SPAWN_MARGIN,
};


// use bevy::prelude::*;
// use bevy::window::PrimaryWindow;
// use bevy::audio::Volume;

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
        let pos = clamp_to_window(
            random::<f32>() * window.width(),
            random::<f32>() * window.height(),
            window,
            SPAWN_MARGIN,
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
        if translation.x != clamp_to_window(translation.x, 0.0, window, 0.0).x  { enemy.direction.x *= -1.0; dir_changed = true; }
        if translation.y != clamp_to_window(0.0, translation.y, window, 0.0).y  { enemy.direction.y *= -1.0; dir_changed = true; }


        if dir_changed {
            // Pick sound
            let sound: Handle<AudioSource> = if random::<f32>() > 0.5 {
                asset_server.load("sounds/pluck_001.ogg")
            } else {
                asset_server.load("sounds/pluck_002.ogg")
            };

            commands.spawn(
                AudioSourceBundle {
                    source: sound,
                    settings: PlaybackSettings {
                        volume: Volume::new(0.3),
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
        let pos = clamp_to_window(
            random::<f32>() * window.width(),
            random::<f32>() * window.height(),
            window,
            SPAWN_MARGIN,
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




/*
    -------------------------
    ---- Clamp To Window ----
    -------------------------
*/

fn clamp_to_window(x: f32, y: f32, window: &Window, margin: f32) -> Vec3 {
    let x_min = 0.0 + RADIUS + margin;
    let x_max = window.width() - RADIUS - margin;
    let y_min = 0.0 + RADIUS + margin;
    let y_max = window.height() - RADIUS - margin;


    return Vec3::new(
        x.clamp(x_min, x_max),
        y.clamp(y_min, y_max),
        0.0
    );
}
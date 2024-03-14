use rand::prelude::*;
use bevy::{
    audio::Volume, prelude::*, window::PrimaryWindow
};

const ENEMY_SPEED: f32 = 200.0;
const ENEMY_COUNT: usize = 4;
const PLAYER_RADIUS: f32 = 32.0; // Should really make a base actor component but whatever
const ENEMY_RADIUS: f32 = 32.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // Startup
        app.add_systems(Startup, spawn_enemies);

        // Update
        app.add_systems(Update, (
            move_enemies,
            update_enemy_direction,
        ));
    }
}




#[derive(Component)]
struct Enemy {
    direction: Vec2,
}




/*
    -----------------------
    ---- Spawn Enemies ----
    -----------------------
*/

fn spawn_enemies(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_q.get_single().unwrap();

    for _ in 0..ENEMY_COUNT {
        let pos = clamp_to_window(
            random::<f32>() * window.width(),
            random::<f32>() * window.height(),
            window
        );

        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(pos.x, pos.y, 0.0),
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

fn move_enemies(
    mut enemy_q: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>
) {
    for (mut transform, enemy) in enemy_q.iter_mut() {
        
        let direction: Vec3 = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);

        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();

    }
}




/*
    ---------------------------------
    ---- Update Enemy Directions ----
    ---------------------------------
*/

fn update_enemy_direction(
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
        if translation.x != clamp_to_window(translation.x, 0.0, window).x  { enemy.direction.x *= -1.0; dir_changed = true; }
        if translation.y != clamp_to_window(0.0, translation.y, window).y  { enemy.direction.y *= -1.0; dir_changed = true; }


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
    ---------------------------------------
    ---- Detect Collisions With Player ----
    ---------------------------------------
*/

// TODO: Implement this lol



/*
    -------------------------
    ---- Clamp To Window ----
    -------------------------
*/

fn clamp_to_window(x: f32, y: f32, window: &Window) -> Vec3 {
    let x_min = 0.0 + ENEMY_RADIUS;
    let x_max = window.width() - ENEMY_RADIUS;
    let y_min = 0.0 + ENEMY_RADIUS;
    let y_max = window.height() - ENEMY_RADIUS;


    return Vec3::new(
        x.clamp(x_min, x_max),
        y.clamp(y_min, y_max),
        0.0
    );
}
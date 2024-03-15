use rand::prelude::*;
use crate::{
    player::Player,
    game::{GameOver, Score},
};
use bevy::{
    prelude::*,
    window::PrimaryWindow,
    audio::Volume,
};



const ENEMY_SPEED: f32 = 200.0;
const ENEMY_RADIUS: f32 = 32.0;
const ENEMY_COUNT: usize = 4;
const ENEMY_Z: f32 = 1.0;

const SPAWN_TIME: f32 = 3.0;

const PLAYER_RADIUS: f32 = 32.0; // Should really make a base actor component but whatever
const SPAWN_MARGIN: f32 = 16.0;




/*
    ---------------------
    ---- Entry Point ----
    ---------------------
*/

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // Startup
        app.add_systems(Startup, spawn_enemies);

        // Update
        app.add_systems(Update, (
            move_enemies,
            update_enemy_direction,
            collide_with_player,
            tick_spawn_timer,
            spawn_over_time,
        ));

        // Resources
        app.init_resource::<EnemySpawnTimer>();
    }
}




/*
    --------------------
    ---- Components ----
    --------------------
*/

#[derive(Component)]
struct Enemy {
    direction: Vec2,
}




/*
    -------------------
    ---- Resources ----
    -------------------
*/

#[derive(Resource)]
struct EnemySpawnTimer {
    timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME, TimerMode::Repeating)
        }
    }
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
            window,
            SPAWN_MARGIN,
        );

        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(pos.x, pos.y, ENEMY_Z),
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
    ---------------------------------------
    ---- Detect Collisions With Player ----
    ---------------------------------------
*/

fn collide_with_player(
    mut commands: Commands,
    mut player_q: Query<(Entity, &Transform), With<Player>>,
    mut game_over_ew: EventWriter<GameOver>,
    enemy_q: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((p_entity, p_transform)) = player_q.get_single_mut() {
        for e_transform in enemy_q.iter() {

            let distance = p_transform.translation.distance(e_transform.translation);
            if distance < PLAYER_RADIUS + ENEMY_RADIUS {

                // Play explosion sound
                commands.spawn(
                    AudioSourceBundle {
                        source: asset_server.load("sounds/explosionCrunch_000.ogg") as Handle<AudioSource>,
                        ..default()
                    }
                );


                // Despawn Player
                commands.entity(p_entity).despawn();


                // Game over
                game_over_ew.send(GameOver { score: score.value });

            }
        }
    }
}




/*
    --------------------------
    ---- Tick Spawn Timer ----
    --------------------------
*/

fn tick_spawn_timer(
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

fn spawn_over_time(
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
                transform: Transform::from_xyz(pos.x, pos.y, ENEMY_Z),
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
    let x_min = 0.0 + ENEMY_RADIUS + margin;
    let x_max = window.width() - ENEMY_RADIUS - margin;
    let y_min = 0.0 + ENEMY_RADIUS + margin;
    let y_max = window.height() - ENEMY_RADIUS - margin;


    return Vec3::new(
        x.clamp(x_min, x_max),
        y.clamp(y_min, y_max),
        0.0
    );
}
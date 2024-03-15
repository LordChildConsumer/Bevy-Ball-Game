use rand::prelude::*;
use crate::player::Player;
use bevy::{
    prelude::*,
    window::PrimaryWindow,
    audio::Volume,
};


const PLAYER_RADIUS: f32 = 32.0;

const STAR_COUNT: i32 = 10;
const STAR_RADIUS: f32 = 15.0;
const SPAWN_TIME: f32 = 1.0;




/*
    ---------------------
    ---- Entry Point ----
    ---------------------
*/

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        // Startup
        app.add_systems(Startup, spawn_stars);

        // Update
        app.add_systems(Update, (
            collide_with_player,
            update_score,
            tick_spawn_timer,
            spawn_over_time,
        ));


        // Resources
        app.init_resource::<Score>();
        app.init_resource::<StarSpawnTimer>();
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
    -------------------
    ---- Resources ----
    -------------------
*/

#[derive(Resource)]
struct Score {
    value: u32,
}

impl Default for Score {
    fn default() -> Score {
        Score {
            value: 0,
        }
    }
}


#[derive(Resource)]
struct StarSpawnTimer {
    timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        StarSpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME, TimerMode::Repeating)
        }
    }
}




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
            Star {},
        ));

    }
}




/*
    ---------------------------------------
    ---- Detect Collisions With Player ----
    ---------------------------------------
*/

fn collide_with_player(
    mut commands: Commands,
    player_q: Query<&Transform, With<Player>>,
    star_q: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {

    if let Ok(p_transform) = player_q.get_single() {
        for (s_entity, s_transform) in star_q.iter() {

            let distance = p_transform.translation.distance(s_transform.translation);

            if distance <= STAR_RADIUS + PLAYER_RADIUS {

                // Increment score
                score.value += 1;

                // Play sound
                commands.spawn(
                    AudioSourceBundle { 
                        source: asset_server.load("sounds/laserLarge_000.ogg") as Handle<AudioSource>,
                        settings: PlaybackSettings {
                            volume: Volume::new(0.5),
                            ..default()
                        },
                        ..default()
                    }
                );

                // Despawn Star
                commands.entity(s_entity).despawn();

            }

        }
    }

}




/*
    ----------------------
    ---- Update Score ----
    ----------------------
*/

fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}




/*
    --------------------------
    ---- Tick Spawn Timer ----
    --------------------------
*/

fn tick_spawn_timer(
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

fn spawn_over_time(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    spawn_timer: Res<StarSpawnTimer>,
) {
    if spawn_timer.timer.finished() {
        let window = window_q.get_single().unwrap();

        // Get position
        let pos = clamp_to_window(
            random::<f32>() * window.width(),
            random::<f32>() * window.height(),
            window
        );

        // Spawn Star
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
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
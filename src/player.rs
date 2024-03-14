use bevy::{
    prelude::*,
    window::PrimaryWindow,
};


// Player Stuff
const PLAYER_SPEED: f32 = 500.0;
const PLAYER_RADIUS: f32 = 32.0;


/*
    ----------------------------
    ---- Plugin Entry Point ----
    ----------------------------
*/

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Startup
        app.add_systems(Startup, spawn_player);
        
        // Update
        app.add_systems(Update, (
            move_player,
            confine_player.after(move_player),
        ));
    }
}




#[derive(Component)]
struct Player;




/*
    ----------------------------
    ---- Player Constructor ----
    ----------------------------
*/

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




/*
    -------------------------
    ---- Player Movement ----
    -------------------------
*/


fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<&mut Transform, With<Player>>
) {
    if let Ok(mut p_transform) = player_q.get_single_mut() {

        // Input
        let mut wish_move = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp)    { wish_move.y += 1.0; } // Up
        if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown)  { wish_move.y -= 1.0; } // Down
        if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) { wish_move.x += 1.0; } // Right
        if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft)  { wish_move.x -= 1.0; } // Left

        // Apply speed
        wish_move = wish_move.normalize_or_zero();
        wish_move *= PLAYER_SPEED;

        // Move
        p_transform.translation += wish_move * time.delta_seconds();

    }
}




/*
    ------------------------
    ---- Confine Player ----
    ------------------------
*/

fn confine_player(
    mut player_q: Query<&mut Transform, With<Player>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut p_transform) = player_q.get_single_mut() {

        let window = window_q.get_single().unwrap();
        
        // Boundaries
        let x_min = 0.0 + PLAYER_RADIUS;
        let x_max = window.width() - PLAYER_RADIUS;
        let y_min = 0.0 + PLAYER_RADIUS;
        let y_max = window.height() - PLAYER_RADIUS;

        let mut translation = p_transform.translation;

        // Confine player
        translation.x = translation.x.clamp(x_min, x_max);
        translation.y = translation.y.clamp(y_min, y_max);


        p_transform.translation = translation;
    }
}
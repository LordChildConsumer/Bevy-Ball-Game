use bevy::{
    prelude::*,
    window::PrimaryWindow,
    audio::Volume,
};



use super::{
    components::*,

    // Constants
    SPEED, RADIUS, Z_INDEX,
    
};

use crate::{
    enemy::components::*, events::*, game::resources::*, star::components::*, utils::clamp_to_window
};




/*
    ----------------------
    ---- Spawn Player ----
    ----------------------
*/

pub fn spawn_player(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_q.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, Z_INDEX),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}




/*
    -------------------------
    ---- Player Movement ----
    -------------------------
*/

pub fn move_player(
    mut player_q: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    window_q: Query<&Window, With<PrimaryWindow>>,
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
        wish_move *= SPEED;

        // Move and confine to window
        let target_pos = p_transform.translation + wish_move * time.delta_seconds();
        p_transform.translation = clamp_to_window(
            target_pos.x,
            target_pos.y,
            RADIUS,
            window_q.get_single().unwrap()
        );

    }
}




/*
    ----------------------------
    ---- Collide With Enemy ----
    ----------------------------
*/

pub fn collide_with_enemy(
    mut commands: Commands,
    mut player_q: Query<(Entity, &Transform), With<Player>>,
    mut game_over_ew: EventWriter<GameOver>,
    enemy_q: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((p_entity, p_transform)) = player_q.get_single_mut() {
        for e_transform in enemy_q.iter() {

            // Distance check
            let distance = p_transform.translation.distance(e_transform.translation);
            if distance < RADIUS + crate::enemy::RADIUS {

                // Play explosion sound
                commands.spawn(
                    AudioSourceBundle {
                        source: asset_server.load("sounds/player_death.ogg") as Handle<AudioSource>,
                        settings: PlaybackSettings { volume: Volume::new(0.3), ..default() }
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
    ---------------------------
    ---- Collide With Star ----
    ---------------------------
*/

pub fn collide_with_star(
    mut commands: Commands,
    mut score: ResMut<Score>,
    player_q: Query<&Transform, With<Player>>,
    star_q: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
) {

    if let Ok(p_transform) = player_q.get_single() {
        for (s_entity, s_transform) in star_q.iter() {

            // Distance check
            let distance = p_transform.translation.distance(s_transform.translation);
            if distance <= crate::star::RADIUS + RADIUS {

                // Increment score
                score.value += 1;

                // Play sound
                commands.spawn(
                    AudioSourceBundle { 
                        source: asset_server.load("sounds/collect_star.ogg") as Handle<AudioSource>,
                        settings: PlaybackSettings {
                            volume: Volume::new(0.2),
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
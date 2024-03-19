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
    assets::{PlayerDeathSound, PlayerSprite, StarCollectSound}, events::*, game:: {
        enemy::components::*,
        score::resources::*,
        star::components::*,
    }
    // utils::clamp_to_window,
};




/*
    ----------------------
    ---- Spawn Player ----
    ----------------------
*/

pub fn spawn_player(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    // asset_server: Res<AssetServer>,
    player_sprite: Res<PlayerSprite>,
) {
    let window = window_q.get_single().unwrap();
    let sprite = player_sprite.0.clone();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, Z_INDEX),
            texture: sprite, //asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}




/*
    ------------------------
    ---- Despawn Player ----
    ------------------------
*/

pub fn despawn_player(
    mut commands: Commands,
    player_q: Query<Entity, With<Player>>,
) {
    for p_entity in player_q.iter() {
        commands.entity(p_entity).despawn();
    }
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
    // window_q: Query<&Window, With<PrimaryWindow>>,
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

        // Move
        p_transform.translation += wish_move * time.delta_seconds();

        // Move and confine to window
        // let target_pos = p_transform.translation + wish_move * time.delta_seconds();
        // p_transform.translation = clamp_to_window(
        //     target_pos.x,
        //     target_pos.y,
        //     RADIUS,
        //     window_q.get_single().unwrap()
        // );

    }
}




/*
    ------------------------
    ---- Confine Player ----
    ------------------------
*/

pub fn confine_player(
    mut player_q: Query<&mut Transform, With<Player>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut p_transform) = player_q.get_single_mut() {

        let window = window_q.get_single().unwrap();
        
        // Boundaries
        let x_min = 0.0 + RADIUS;
        let x_max = window.width() - RADIUS;
        let y_min = 0.0 + RADIUS;
        let y_max = window.height() - RADIUS;

        let mut translation = p_transform.translation;

        // Confine player
        translation.x = translation.x.clamp(x_min, x_max);
        translation.y = translation.y.clamp(y_min, y_max);


        p_transform.translation = translation;
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
    // asset_server: Res<AssetServer>,
    death_sound: Res<PlayerDeathSound>,
    score: Res<Score>,
) {
    if let Ok((p_entity, p_transform)) = player_q.get_single_mut() {
        for e_transform in enemy_q.iter() {

            // Distance check
            let distance = p_transform.translation.distance(e_transform.translation);
            if distance < RADIUS + crate::game::star::RADIUS {

                // Play explosion sound
                let explosion_sound = death_sound.0.clone();
                commands.spawn(
                    AudioSourceBundle {
                        source: explosion_sound, //asset_server.load("sounds/player_death.ogg") as Handle<AudioSource>,
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
    // asset_server: Res<AssetServer>,
    collect_sound: Res<StarCollectSound>,
) {

    if let Ok(p_transform) = player_q.get_single() {
        for (s_entity, s_transform) in star_q.iter() {

            // Distance check
            let distance = p_transform.translation.distance(s_transform.translation);
            if distance <= crate::game::star::RADIUS + RADIUS {

                // Increment score
                score.value += 1;

                let sound = collect_sound.0.clone();
                // Play sound
                commands.spawn(
                    AudioSourceBundle { 
                        source: sound, //asset_server.load("sounds/collect_star.ogg") as Handle<AudioSource>,
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


use crate::events::*;

use super::resources::HighScores;

use bevy::{
    prelude::*,
    window::PrimaryWindow,
    app::AppExit,
};

/*
    ----------------------
    ---- Spawn Camera ----
    ----------------------
*/

pub fn spawn_camera(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_q.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 10.0),
            ..default()
        }
    );
}




/*
    ----------------------------
    ---- Quit Game With Esc ----
    ----------------------------
*/

pub fn quit_game(
    mut app_exit_ew: EventWriter<AppExit>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        app_exit_ew.send(AppExit);
    }
}




/*
    --------------------------
    ---- Handle Game Over ----
    --------------------------
*/

pub fn handle_game_over(
    mut game_over_er: EventReader<GameOver>,
) {
    for event in game_over_er.read() {

        println!("Final Score: {}", event.score.to_string());

    }
}




/*
    ----------------------------
    ---- Update High Scores ----
    ----------------------------
*/

pub fn update_high_scores(
    mut game_over_er: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_er.read() {

        // TODO: Customizable player name
        high_scores.scores.push(("Player".to_string(), event.score));

    }
}




/*
    ----------------------------------
    ---- Check High Score Updates ----
    ----------------------------------
*/

pub fn high_scores_updated(
    high_scores: Res<HighScores>,
) {
    if high_scores.is_changed() {

        println!("High Scores: {:?}", high_scores);

    }
}
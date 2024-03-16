use bevy::prelude::*;

use crate::{
    events::*,
    AppState,
};

use super::resources::*;




/*
    ----------------------
    ---- Insert Score ----
    ----------------------
*/

pub fn insert_score(
    mut commands: Commands,
) {
    commands.insert_resource(Score::default());
}




/*
    ----------------------
    ---- Remove Score ----
    ----------------------
*/

pub fn remove_score(
    mut commands: Commands,
) {
    commands.remove_resource::<Score>();
}




/*
    --------------------------
    ---- Handle Game Over ----
    --------------------------
*/

pub fn handle_game_over(
    mut next_state: ResMut<NextState<AppState>>,
    mut game_over_er: EventReader<GameOver>,
) {
    for event in game_over_er.read() {

        println!("Final Score: {}", event.score.to_string());
        next_state.set(AppState::GameOver);

    }
}




/*
    ----------------------
    ---- Update Score ----
    ----------------------
*/

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
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
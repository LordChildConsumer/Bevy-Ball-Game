use bevy::prelude::*;

use crate::events::GameOver;
use crate::game::ui::game_over::components::FinalScoreText;




/*
    ----------------------------
    ---- Update Final Score ----
    ----------------------------
*/

pub fn update_final_score_text(
    mut game_over_er: EventReader<GameOver>,
    mut text_q: Query<&mut Text, With<FinalScoreText>>,
) {
    for event in game_over_er.read() {
        for mut text in text_q.iter_mut() {
            text.sections[0].value = format!("Final Score: {}", event.score.to_string());
        }
    }
}
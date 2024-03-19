use bevy::prelude::*;

use crate::game::{
    enemy::components::Enemy,
    score::resources::Score,
    ui::hud::components::{EnemyText, ScoreText}
};




/*
    ---------------------------
    ---- Update Score Text ----
    ---------------------------
*/

pub fn update_score_text(
    mut text_q: Query<&mut Text, With<ScoreText>>,
    score: Res<Score>,
) {
    if score.is_changed() {
        for mut text in text_q.iter_mut() {
            text.sections[0].value = format!("{}", score.value.to_string());
        }
    }
}




/*
    ---------------------------
    ---- Update Enemy Text ----
    ---------------------------
*/

pub fn update_enemy_text(
    mut text_q: Query<&mut Text, With<EnemyText>>,
    enemy_q: Query<Entity, With<Enemy>>,
) {
    let count = enemy_q.iter().count();
    for mut text in text_q.iter_mut() {
        text.sections[0].value = format!("{}", count.to_string());
    }
}
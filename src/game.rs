use bevy::{
    prelude::*,
    app::AppExit,
};




/*
    ---------------------
    ---- Entry Point ----
    ---------------------
*/

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Update
        app.add_systems(Update, (
            quit_game,
            handle_game_over,
            update_high_scores,
            high_scores_updated,
        ));

        // Events
        app.add_event::<GameOver>();

        // Resources
        app.init_resource::<Score>();
        app.init_resource::<HighScores>();
    }
}




/*
    -------------------
    ---- Resources ----
    -------------------
*/

// Score
#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Score {
        Score {
            value: 0,
        }
    }
}


// High Scores
#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScores {
    fn default() -> HighScores {
        HighScores {
            scores: Vec::new(),
        }
    }
}




/*
    ----------------
    ---- Events ----
    ----------------
*/

// Game Over
#[derive(Event)]
pub struct GameOver {
    pub score: u32,
}




/*
    ----------------------------
    ---- Quit Game With Esc ----
    ----------------------------
*/

fn quit_game(
    keys: Res<ButtonInput<KeyCode>>,
    mut app_exit_ew: EventWriter<AppExit>,
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

fn handle_game_over(
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

fn update_high_scores(
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

fn high_scores_updated(
    high_scores: Res<HighScores>,
) {
    if high_scores.is_changed() {

        println!("High Scores: {:?}", high_scores);

    }
}
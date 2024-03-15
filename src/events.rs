use bevy::prelude::*;

// Game Over
#[derive(Event)]
pub struct GameOver {
    pub score: u32,
}

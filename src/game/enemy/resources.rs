use bevy::prelude::*;


use super::SPAWN_TIME;


#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME, TimerMode::Repeating)
        }
    }
}
use bevy::prelude::*;

const SPAWN_TIME: f32 = 1.0;


#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        StarSpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME, TimerMode::Repeating)
        }
    }
}
use std::time::Duration;

use bevy::prelude::*;

use crate::AppState;

use super::level_end_animation::AnimationFinishedEvent;
pub struct LevelPlugin;

#[derive(Event)]
pub struct LevelFinishedEvent;

// TODO: I don't want to have Default
// TODO: Should this be Clone?
#[derive(Default, Clone)]
pub struct LevelConfig {
    pub duration: Duration,
}

impl LevelConfig {
    pub const fn from_seconds_duration(seconds: u64) -> Self {
        Self {
            duration: Duration::from_secs(seconds),
        }
    }
}

#[derive(Resource, Default)]
pub struct CurrentLevelConfig(pub LevelConfig);

#[derive(Event)]
pub struct CombatFinishedEvent;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentLevelConfig>() // TODO: There is no initial value :(
            .add_event::<LevelFinishedEvent>()
            .add_event::<CombatFinishedEvent>()
            .add_systems(Update, (on_finished_combat, on_finished_animation));
    }
}

fn on_finished_combat(
    mut combat_finished: EventReader<CombatFinishedEvent>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for _ in combat_finished.read() {
        next_state.set(AppState::LevelEndAnimation);
    }
}

fn on_finished_animation(
    mut animation_finished: EventReader<AnimationFinishedEvent>,
    mut level_finished: EventWriter<LevelFinishedEvent>,
) {
    for _ in animation_finished.read() {
        level_finished.send(LevelFinishedEvent);
    }
}

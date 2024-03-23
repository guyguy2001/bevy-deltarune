use bevy::prelude::*;

use crate::AppState;
pub struct LevelPlugin;

#[derive(Event)]
pub struct LevelFinishedEvent;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelFinishedEvent>()
            .add_systems(Update, on_finished_level);
    }
}

fn on_finished_level(
    mut level_finished: EventReader<LevelFinishedEvent>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for _ in level_finished.read() {
        next_state.set(AppState::LevelEndAnimation);
    }
}

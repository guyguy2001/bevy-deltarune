use bevy::prelude::*;

use crate::AppState;

use super::level_end_animation::AnimationFinishedEvent;
pub struct LevelPlugin;

#[derive(Event)]
pub struct CombatFinishedEvent;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CombatFinishedEvent>()
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
    mut next_state: ResMut<NextState<AppState>>,
) {
    for _ in animation_finished.read() {
        next_state.set(AppState::LevelTransition);
    }
}

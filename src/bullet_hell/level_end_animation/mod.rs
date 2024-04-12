use bevy::prelude::*;

use crate::AppState;

use super::effects::{effect::LevelTransitionEffectsPool, spawning_animation::SpawningAnimation};

pub struct LevelEndAnimationPlugin;

/// When the level is finished, we first play an animation of whatever happens at the level transition
/// (currently: a new cannon gets spawned), and then wait for an additional time to let the user take that in.
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum LevelEndAnimationState {
    #[default]
    Animations,
    PostAnimationsDelay,
}

#[derive(Resource)]
struct PostAnimationsDelayTimer(Timer);

impl PostAnimationsDelayTimer {
    fn new() -> Self {
        Self(Timer::from_seconds(1., TimerMode::Once))
    }
}

impl Plugin for LevelEndAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(LevelEndAnimationState::Animations)
            .insert_resource(PostAnimationsDelayTimer::new())
            .add_systems(
                OnEnter(AppState::LevelEndAnimation),
                (reset_animation_state, apply_random_effect),
            )
            .add_systems(
                Update,
                (
                    check_for_animations_end
                        .run_if(in_state(AppState::LevelEndAnimation))
                        .run_if(in_state(LevelEndAnimationState::Animations)),
                    post_animations_delay
                        .run_if(in_state(AppState::LevelEndAnimation))
                        .run_if(in_state(LevelEndAnimationState::PostAnimationsDelay)),
                ),
            );
    }
}

fn reset_animation_state(mut animation_state: ResMut<NextState<LevelEndAnimationState>>) {
    animation_state.0 = Some(LevelEndAnimationState::Animations);
}

fn apply_random_effect(effects: Res<LevelTransitionEffectsPool>, mut commands: Commands) {
    commands.run_system(effects.get_random().0);
}

fn check_for_animations_end(
    q_active_animations: Query<(), With<SpawningAnimation>>,
    mut animation_state: ResMut<NextState<LevelEndAnimationState>>,
) {
    if q_active_animations.is_empty() {
        animation_state.0 = Some(LevelEndAnimationState::PostAnimationsDelay);
    }
}

fn post_animations_delay(
    time: Res<Time>,
    mut timer: ResMut<PostAnimationsDelayTimer>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        timer.0.reset();
        app_state.0 = Some(AppState::LevelTransition);
    }
}

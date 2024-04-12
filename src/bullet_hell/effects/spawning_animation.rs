use std::time::Duration;

use bevy::prelude::*;
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};

pub struct SpawningAnimationPlugin;

impl Plugin for SpawningAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawning_animation_behaviour);
    }
}

// TODO: I want to play a spawning animation after the finishing of a level.
// I might want to give the effect a timer, I might want to give the spawning it self a timer, and I might want to hav a global "TrunEndAnimations" timer. IDK yet.
#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct SpawningAnimation {
    pub animation_timer: Timer,
    // TODO: Do I want a reference to the state it's relevant in? maybe that should be a component?
}

impl SpawningAnimation {
    pub fn new(animation_duration: Duration) -> Self {
        Self {
            animation_timer: Timer::new(animation_duration, TimerMode::Once),
        }
    }
}

fn spawning_animation_behaviour(
    mut q_spawning_object: Query<(Entity, &mut Sprite, &mut SpawningAnimation)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut sprite, mut animation) in q_spawning_object.iter_mut() {
        sprite
            .color
            .set_a((0.).lerp(1., animation.animation_timer.fraction()));

        animation.animation_timer.tick(time.delta());
        if animation.animation_timer.finished() {
            commands.entity(entity).remove::<SpawningAnimation>();
        }
    }
}

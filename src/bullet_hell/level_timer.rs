use std::time::Duration;

use bevy::{math::vec3, prelude::*};

use crate::{
    utils::{world_ui::WorldUI, z_index},
    AppState,
};

use super::level::LevelFinishedEvent;

pub struct LevelTimerPlugin;

impl Plugin for LevelTimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_timer)
            .add_systems(OnEnter(AppState::Defending), reset_timer)
            .add_systems(
                Update,
                timer_behaviour.run_if(in_state(AppState::Defending)),
            );
    }
}

#[derive(Component)]
struct LevelTimer {
    remaining_time: Timer,
}

impl LevelTimer {
    pub fn new(duration: Duration) -> Self {
        Self {
            remaining_time: Timer::new(duration, TimerMode::Once),
        }
    }
}

fn spawn_timer(mut commands: Commands) {
    // TODO: Do I want to export a `spawn_at_position` function from the world_ui module?
    let world_position = vec3(-50., 65.5, 0.);
    let axis_entity = commands
        .spawn((
            TransformBundle {
                local: Transform::from_translation(world_position),
                ..Default::default()
            },
            Name::new("LevelTimer Axis"),
        ))
        .id();

    commands
        .spawn((
            NodeBundle {
                z_index: z_index::WORLD_UI,
                style: Style {
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            WorldUI {
                tracked_entity: axis_entity,
            },
            Name::new("LevelTimer WorldUI"),
        ))
        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "5.00",
                        TextStyle {
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    ..Default::default()
                },
                LevelTimer::new(Duration::from_secs(5)),
                Name::new("LevelTimer"),
            ));
        });
}

fn timer_behaviour(
    mut q_timer: Query<(&mut LevelTimer, &mut Text)>,
    mut win_event: EventWriter<LevelFinishedEvent>,
    time: Res<Time>,
) {
    for (mut timer, mut text) in q_timer.iter_mut() {
        timer.remaining_time.tick(time.delta());
        let remaining_time = timer.remaining_time.remaining_secs();
        text.sections[0].value = format!("{remaining_time:.2}");

        if timer.remaining_time.just_finished() {
            win_event.send(LevelFinishedEvent);
        }
    }
}

fn reset_timer(mut q_timer: Query<&mut LevelTimer>) {
    for mut timer in q_timer.iter_mut() {
        timer.remaining_time.reset();
    }
}

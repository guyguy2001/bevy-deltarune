use bevy::prelude::*;

use crate::{
    bullet_hell::{CurrentLevelConfig, LevelConfig, LevelFinishedEvent},
    ui::level_transition::FinishedLevelTransitionEvent,
    AppState,
};

pub struct MetagamePlugin;

impl Plugin for MetagamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartGameEvent>()
            .insert_resource(MetagameProgression {
                levels: Vec::from(GAME_STEPS),
                current_step_index: 0,
                current_level: 0,
            })
            .add_systems(Startup, ui::spawn_menu)
            .add_systems(
                Update,
                (
                    start_game.run_if(on_event::<StartGameEvent>()), // TODO: This shouldn't be an event...
                    on_finished_step.run_if(on_event::<LevelFinishedEvent>()),
                    on_finished_step.run_if(on_event::<FinishedLevelTransitionEvent>()),
                    ui::update_text_on_level_transition,
                ),
            );
    }
}

#[derive(Event)]
pub struct StartGameEvent;

#[derive(Clone)] // TODO - should this be clone?
enum GameSteps {
    Level(LevelConfig),
    UpgradeShop,
}

const GAME_STEPS: [GameSteps; 6] = [
    GameSteps::Level(LevelConfig::from_seconds_duration(15)),
    GameSteps::UpgradeShop,
    GameSteps::Level(LevelConfig::from_seconds_duration(10)),
    GameSteps::Level(LevelConfig::from_seconds_duration(10)),
    GameSteps::UpgradeShop,
    GameSteps::Level(LevelConfig::from_seconds_duration(30)),
];

// TODO: My idea was to track the between-levels progression here, but in actuality the bullet_hell
// module handles a lot of state that is retained between levels, and behaviour that is _about_
// level transition (specifically the end-of-level enemy spawn events).
// What is this plugin supposed to handle, and what should be handled in the bullet_hell module?

// Well, for one, this plugin doesn't actually handle the levels transition... yeah

#[derive(Resource)]
struct MetagameProgression {
    current_step_index: usize,
    current_level: usize,
    levels: Vec<GameSteps>,
}

fn start_game(
    mut progression: ResMut<MetagameProgression>,
    mut next_state: ResMut<NextState<AppState>>,
    mut current_level_config: ResMut<CurrentLevelConfig>,
) {
    match &progression.levels[progression.current_step_index] {
        GameSteps::Level(level_config) => {
            current_level_config.0 = level_config.clone();
            next_state.0 = Some(AppState::Defending);

            progression.current_level += 1;
        }
        GameSteps::UpgradeShop => {
            next_state.0 = Some(AppState::LevelTransition);
        }
    }
}

fn on_finished_step(
    mut progression: ResMut<MetagameProgression>,
    mut next_state: ResMut<NextState<AppState>>,
    mut current_level_config: ResMut<CurrentLevelConfig>,
) {
    if progression.current_step_index == progression.levels.len() - 1 {
        println!("You win!");
        next_state.0 = Some(AppState::Victory);
        return;
    }

    progression.current_step_index += 1;
    println!("Transitioned to step {}", progression.current_step_index);

    match &progression.levels[progression.current_step_index] {
        GameSteps::Level(level_config) => {
            current_level_config.0 = level_config.clone();
            next_state.0 = Some(AppState::Defending);

            progression.current_level += 1;
        }
        GameSteps::UpgradeShop => {
            next_state.0 = Some(AppState::LevelTransition);
        }
    }
}

mod ui {
    // use crate::utils::sickle::ui_builder::{UiBuilderExt, UiContextRoot, UiRoot};
    // use crate::utils::sickle::ui_commands::SetCursorExt;
    use bevy::prelude::*;
    use bevy_inspector_egui::prelude::*;

    use super::MetagameProgression;

    // use crate::utils::sickle::ui_style::{SetBackgroundColorExt, SetNodeHeightExt, SetNodeWidthExt};
    #[derive(Component, InspectorOptions, Default, Reflect)]
    #[reflect(Component, InspectorOptions)]
    pub struct LevelText;

    pub fn spawn_menu(mut commands: Commands) {
        commands
            // .ui_builder(UiRoot)
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(50.),
                    left: Val::Px(25.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|builder| {
                builder.spawn((
                    TextBundle {
                        text: Text::from_section(
                            "Level 1",
                            TextStyle {
                                font_size: 32.0,
                                ..default()
                            },
                        ),
                        ..Default::default()
                    },
                    LevelText,
                ));
            });

        //    commands .ui_builder(UiRoot).
    }

    pub fn update_text_on_level_transition(
        mut query: Query<&mut Text, With<LevelText>>,
        progress: Res<MetagameProgression>,
    ) {
        if progress.is_changed() {
            for mut text in query.iter_mut() {
                text.sections[0].value = format!("Level {}", progress.current_level);
            }
        }
    }
}

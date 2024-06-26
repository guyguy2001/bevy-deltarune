use bevy::prelude::*;

use crate::{
    bullet_hell::{CurrentLevelConfig, LevelConfig, LevelFinishedEvent},
    ui::level_transition::FinishedLevelTransitionEvent,
    AppState,
};

use super::ui;

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

#[derive(Resource)]
pub struct MetagameProgression {
    levels: Vec<GameSteps>,
    current_step_index: usize,

    pub current_level: usize,
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

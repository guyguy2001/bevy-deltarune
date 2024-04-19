use std::fs;

use bevy::{prelude::*, render::camera::ScalingMode, window::WindowMode};
use bevy_editor_pls::prelude::*;

// use bevy_inspector_egui_rapier::InspectableRapierPlugin;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;

use bullet_hell::BulletHellPlugin;
use game_config::GameConfig;
use level_transition::LevelTransitionPlugin;
use lose_screen::LoseScreenPlugin;
use menu::MenuUI;
use upgrades::UpgradesPlugin;
use utils::{menu_system::MenuSystemPlugin, world_ui::WorldUIPlugin};

mod bullet_hell;
mod game_config;
mod level_transition;
mod lose_screen;
mod menu;
mod upgrades;
mod utils;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default, Deserialize)]
pub enum AppState {
    #[default]
    ActionMenu,
    Defending,
    LevelEndAnimation,
    LevelTransition,
    Defeat,
}

fn main() {
    let game_config = get_config();
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Gems game".into(),
                        resolution: game_config.window_size.into(),
                        mode: if game_config.fullscreen {
                            WindowMode::BorderlessFullscreen
                        } else {
                            WindowMode::Windowed
                        },
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(RapierDebugRenderPlugin {
            mode: DebugRenderMode::empty(),
            ..Default::default()
        })
        .add_plugins(EditorPlugin::default())
        // .add_plugins(InspectableRapierPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .insert_state::<AppState>(
            game_config
                .debug
                .as_ref()
                .map_or(AppState::default(), |conf| conf.starting_state),
        )
        .insert_resource(game_config)
        .add_plugins(WorldUIPlugin)
        .add_plugins(BulletHellPlugin)
        .add_plugins(LevelTransitionPlugin)
        // .add_plugins(
        //     WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        // )
        .add_plugins(MenuSystemPlugin)
        .add_plugins(MenuUI)
        .add_plugins(LoseScreenPlugin)
        .add_plugins(UpgradesPlugin) // TODO: Should this be here?
        .insert_resource(Money(100.0))
        .add_systems(Startup, setup_camera)
        .run();
}

fn get_config() -> GameConfig {
    let config_string: String = fs::read_to_string("assets/config.ron").unwrap();
    let game_config: GameConfig = ron::de::from_str(&config_string).unwrap_or_else(|e| {
        println!("Failed to load config: {}", e);
        std::process::exit(1);
    });
    game_config
}

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::FixedVertical(200.);

    commands.spawn((camera, IsDefaultUiCamera, Name::new("Camera")));
}

#[derive(Resource)]
pub struct Money(pub f32);

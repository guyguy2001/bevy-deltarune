use std::fs;

use bevy::{
    input::common_conditions::input_toggle_active, prelude::*, render::camera::ScalingMode,
    window::WindowMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use bevy_inspector_egui_rapier::InspectableRapierPlugin;
use bevy_rapier2d::prelude::*;

use bullet_hell::BulletHellPlugin;
use game_config::GameConfig;
use level_transition::LevelTransitionPlugin;
use lose_screen::LoseScreenPlugin;
use menu::MenuUI;
use utils::{menu_system::MenuSystemPlugin, world_ui::WorldUIPlugin};

mod bullet_hell;
mod game_config;
mod level_transition;
mod lose_screen;
mod menu;
mod utils;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
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
        // .add_plugins(InspectableRapierPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .init_state::<AppState>()
        .add_plugins(WorldUIPlugin)
        .add_plugins(BulletHellPlugin)
        .add_plugins(LevelTransitionPlugin)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_plugins(MenuSystemPlugin)
        .add_plugins(MenuUI)
        .add_plugins(LoseScreenPlugin)
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

    commands.spawn((camera, Name::new("Camera")));
}

#[derive(Resource)]
pub struct Money(pub f32);

use std::fs;

use bevy::input::common_conditions::input_toggle_active;
use bevy::{prelude::*, render::camera::ScalingMode, window::WindowMode};
// use bevy_editor_pls::prelude::*;

use avian2d::debug_render::PhysicsDebugPlugin;
use avian2d::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use bevy_tnua::prelude::*;
// use bevy_tnua_avian2d::TnuaAvian2dPlugin;
use bevy_tweening::TweeningPlugin;
use metagame::MetagamePlugin;
use serde::Deserialize;

use bullet_hell::BulletHellPlugin;
use game_config::GameConfig;
use ui::lose_screen::LoseScreenPlugin;
use ui::menu::MenuUI;
use ui::{level_transition::LevelTransitionPlugin, victory_screen::VictoryScreenPlugin};
use upgrades::UpgradesPlugin;
use utils::{menu_system::MenuSystemPlugin, world_ui::WorldUIPlugin};

mod abilities;
mod bullet_hell;
mod game_config;
mod metagame;
mod ui;
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
    Victory,
}

fn main() {
    let game_config = get_config();
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Gems game".into(),
                    resolution: game_config.window_size.into(),
                    present_mode: bevy::window::PresentMode::AutoNoVsync,
                    mode: if game_config.fullscreen {
                        WindowMode::BorderlessFullscreen(MonitorSelection::Primary)
                    } else {
                        WindowMode::Windowed
                    },
                    resizable: true,
                    ..default()
                }),
                ..default()
            }),
    )
    .add_plugins(PhysicsPlugins::new(FixedPostUpdate))
    // .add_plugins(TnuaAvian2dPlugin::new(FixedUpdate))
    .add_plugins(TweeningPlugin)
    // .add_plugins(RapierDebugRenderPlugin {
    //     mode: DebugRenderMode::empty(),
    //     ..Default::default()
    // })
    .add_plugins(EguiPlugin {
        enable_multipass_for_primary_context: true,
    })
    .add_plugins(WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::Escape)))
    // .add_plugins(EditorPlugin::default())
    // .add_plugins(InspectableRapierPlugin)
    // .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .insert_state::<AppState>(
        game_config
            .debug
            .as_ref()
            .map_or(AppState::default(), |conf| conf.starting_state),
    )
    .add_plugins(WorldUIPlugin)
    .add_plugins(BulletHellPlugin)
    .add_plugins(MetagamePlugin)
    .add_plugins(LevelTransitionPlugin)
    // .add_plugins(
    //     WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
    // )
    .add_plugins(MenuSystemPlugin)
    .add_plugins(MenuUI)
    .add_plugins(LoseScreenPlugin)
    .add_plugins(VictoryScreenPlugin)
    .add_plugins(UpgradesPlugin) // TODO: Should this be here?
    .add_plugins(utils::kinematic_controller::plugin)
    .insert_resource(Money(100.0))
    .add_systems(Startup, setup_camera);
    if game_config.physics_gizmos {
        app.add_plugins(PhysicsDebugPlugin::default());
    }
    app.insert_resource(game_config);
    app.run();
}

fn get_config() -> GameConfig {
    let config_string: String = fs::read_to_string("assets/config.ron").unwrap();
    let game_config: GameConfig = ron::de::from_str(&config_string).unwrap_or_else(|e| {
        error!("Failed to load config: {}", e);
        std::process::exit(1);
    });
    game_config
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 200.,
            },
            ..OrthographicProjection::default_2d()
        }),
        IsDefaultUiCamera,
        Name::new("Camera"),
    ));
}

#[derive(Resource)]
pub struct Money(pub f32);

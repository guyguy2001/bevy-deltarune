use bevy::{
    input::common_conditions::input_toggle_active, prelude::*, render::camera::ScalingMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use bevy_inspector_egui_rapier::InspectableRapierPlugin;
use bevy_rapier2d::prelude::*;

use bullet_hell::BulletHellPlugin;
use level_transition::LevelTransitionPlugin;
use lose_screen::LoseScreenPlugin;
use menu::MenuUI;
use utils::world_ui::WorldUIPlugin;

mod bullet_hell;
mod level_transition;
mod lose_screen;
mod menu;
mod utils;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    ActionMenu,
    Attacking,
    Defending,
    Defeat,
    LevelTransition,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Gems game".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
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
        .add_state::<AppState>()
        .add_plugins(WorldUIPlugin)
        .add_plugins(BulletHellPlugin)
        .add_plugins(LevelTransitionPlugin)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_plugins(MenuUI)
        .add_plugins(LoseScreenPlugin)
        .insert_resource(Money(100.0))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn((camera, Name::new("Camera")));
}

#[derive(Resource)]
pub struct Money(pub f32);

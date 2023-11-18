use bevy::{
    input::common_conditions::input_toggle_active, prelude::*, render::camera::ScalingMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bullet_hell::{bullet::BulletsPlugin, player::PlayerPlugin};
use menu::MenuUI;
use ui::GameUI;

mod bullet_hell;
mod menu;
mod ui;
mod utils;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    ActionMenu,
    Attacking,
    Defending,
}

fn main() {
    let mut x = 3;
    let mut y = 5;
    let mut z = &mut x;
    *z = 1;
    z = &mut y;
    *z = 2;
    println!("{}, {}", x, y);

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
        .add_state::<AppState>()
        .add_plugins((GameUI, BulletsPlugin, PlayerPlugin))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_plugins(MenuUI)
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

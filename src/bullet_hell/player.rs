use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::{
    game_config::GameConfig,
    ui::lose_screen::LoseEvent,
    upgrades::{UpgradesReceiver, UpgradesReceiverFaction},
    utils::{input::get_input_direction, kinematic_controller::KinematicController},
    AppState,
};

use super::{
    dash::Dasher, game_ui::healthbar::spawn_healthbar, game_z_index, health::Health, physics_layers,
};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .add_systems(Startup, setup_player)
            .add_systems(
                Update,
                (character_movement, player_death).run_if(in_state(AppState::Defending)),
            );
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Player {
    #[inspector(min = 0.0)]
    pub speed: f32,
}

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<GameConfig>,
) {
    let sprite_size = 7.5;
    let player_commands = commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(sprite_size, sprite_size))),
        MeshMaterial2d(materials.add(Color::srgb_u8(165, 75, 251))),
        Transform::from_translation(Vec3::Z * game_z_index::PLAYERS),
        Dasher {
            dash_amount: 50.,
            // dash_duration: Duration::from_secs_f32(0.5),
            dash_speed: 200.,
        },
        Player { speed: 100.0 },
        Health::new(if config.infinite_hp { 100000. } else { 20. }),
        UpgradesReceiver {
            factions: UpgradesReceiverFaction::Player,
        },
        Name::new("Player"),
        KinematicController,
        (
            CollisionLayers::new(
                physics_layers::GameLayers::Player,
                physics_layers::GameLayers::all_bits(),
            ),
            Collider::rectangle(sprite_size, sprite_size),
            RigidBody::Kinematic,
            LinearVelocity::default(),
            CollisionEventsEnabled,
        ),
    ));
    let player_entity = player_commands.id();
    spawn_healthbar(&mut commands, player_entity);
}

fn character_movement(
    mut characters: Query<
        (&mut LinearVelocity, &Player),
        (ControllablePlayerFilter, With<KinematicController>),
    >,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (mut velocity, player) in &mut characters {
        velocity.0 = get_input_direction(&input).xy() * player.speed;
    }
}

fn player_death(
    health_query: Query<&Health, With<Player>>,
    mut lose_event: EventWriter<LoseEvent>,
) {
    for health in health_query.iter() {
        if health.health <= 0. {
            lose_event.write(LoseEvent);
        }
    }
}

/// For stuff that take away the input and control the player's movement themselves, like dashes.
#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct ControlledExternally {
    controller_name: &'static str,
}

impl ControlledExternally {
    pub fn new(controller_name: &'static str) -> Self {
        Self { controller_name }
    }
}

// TODO: Idk if this is a good idea, or if it abstracts away what doesn't need to be abstracted;
// But I'm rolling with this for now.
pub type ControllablePlayerFilter = (With<Player>, Without<ControlledExternally>);

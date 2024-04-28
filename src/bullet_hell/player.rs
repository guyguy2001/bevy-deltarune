use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use bevy_rapier2d::prelude::*;

use crate::{
    game_config::GameConfig,
    ui::lose_screen::LoseEvent,
    upgrades::{UpgradesReceiver, UpgradesReceiverFaction},
    utils::input::get_input_direction,
    AppState,
};

use super::{
    dash::{ControlledExternally, Dasher},
    game_ui::healthbar::spawn_healthbar,
    game_z_index,
    health::Health,
    physics_layers,
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
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(sprite_size, sprite_size))),
            material: materials.add(Color::rgb_u8(165, 75, 251)),
            transform: Transform::from_translation(Vec3::Z * game_z_index::PLAYERS),
            ..default()
        },
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
        (
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(physics_layers::PLAYERS, physics_layers::ALL),
            ActiveCollisionTypes::all(),
            Collider::cuboid(sprite_size / 2.0, sprite_size / 2.0),
            KinematicCharacterController {
                filter_flags: QueryFilterFlags::EXCLUDE_SENSORS,
                ..Default::default()
            },
            RigidBody::KinematicPositionBased,
        ),
    ));
    let player_entity = player_commands.id();
    spawn_healthbar(&mut commands, player_entity);
}

fn character_movement(
    mut characters: Query<
        (&mut KinematicCharacterController, &Player),
        Without<ControlledExternally>,
    >,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut controller, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();
        controller.translation = Some(get_input_direction(&input).xy() * movement_amount);
    }
}

fn player_death(
    health_query: Query<&Health, With<Player>>,
    mut lose_event: EventWriter<LoseEvent>,
) {
    for health in health_query.iter() {
        if health.health <= 0. {
            lose_event.send(LoseEvent);
        }
    }
}

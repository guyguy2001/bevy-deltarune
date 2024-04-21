use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use bevy_rapier2d::prelude::*;

use crate::{
    game_config::GameConfig,
    lose_screen::LoseEvent,
    upgrades::{UpgradesReceiver, UpgradesReceiverFaction},
    AppState,
};

use super::{game_ui::healthbar::spawn_healthbar, game_z_index, health::Health};

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
        Player { speed: 100.0 },
        Health::new(if config.infinite_hp { 100000. } else { 20. }),
        UpgradesReceiver {
            factions: UpgradesReceiverFaction::Player,
        },
        Name::new("Player"),
        (
            ActiveEvents::COLLISION_EVENTS,
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
    mut characters: Query<(&mut KinematicCharacterController, &Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut controller, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();
        let mut desired_direction = Vec2::ZERO;
        if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
            desired_direction += Vec2::Y * movement_amount;
        }
        if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
            desired_direction += -Vec2::Y * movement_amount;
        }
        if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
            desired_direction += Vec2::X * movement_amount;
        }
        if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
            desired_direction += -Vec2::X * movement_amount;
        }
        controller.translation = Some(desired_direction.normalize_or_zero() * movement_amount);
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

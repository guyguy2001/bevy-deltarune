use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use bevy_rapier2d::prelude::*;

use crate::{lose_screen::LoseEvent, AppState};

use super::healthbar::{spawn_healthbar, Health};

// TODO: Rename to GameWorldMovement?

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

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("character.png");
    let sprite_size = 7.5;
    let player_commands = commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(sprite_size, sprite_size)),
                ..default()
            },
            texture,
            ..default()
        },
        Player { speed: 100.0 },
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
            Health::new(5.),
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

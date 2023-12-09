use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use bevy_rapier2d::prelude::*;

use crate::AppState;

use super::player::Player;

pub struct BulletsPlugin;

impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_bullets_from_input, player_collision).run_if(in_state(AppState::Defending)),
        );
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
struct Bullet {
    pub direction: Vec3,
}

fn spawn_bullets_from_input(
    commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        spawn_bullets(commands, asset_server);
    }
}

fn spawn_bullets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite_size = 16.0;
    let direction = Vec3::new(200.0, 0.0, 0.0);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(sprite_size, sprite_size)),
                ..default()
            },
            texture: asset_server.load("pig.png"),
            transform: Transform {
                translation: Vec3::new(-50.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        Bullet {
            direction: direction,
        },
        (
            ActiveEvents::COLLISION_EVENTS,
            ActiveCollisionTypes::all(),
            RigidBody::KinematicVelocityBased,
            Velocity {
                linvel: direction.xy(),
                ..Default::default()
            },
            Collider::cuboid(sprite_size / 2.0, sprite_size / 2.0),
            Sensor,
        ),
    ));
}

fn player_collision(
    mut commands: Commands,
    mut contact_events: EventReader<CollisionEvent>,
    bullets: Query<(Entity, With<Bullet>)>,
    players: Query<(Entity, With<Player>)>,
) {
    for event in contact_events.read() {
        println!("{:?}", event);
        if let CollisionEvent::Started(entity1, entity2, _) = event {
            // TODO: get this working with swapped entity orders???
            if players.contains(*entity1) {
                if let Ok((bullet, _)) = bullets.get(*entity2) {
                    commands.entity(bullet).despawn_recursive();
                }
            }
        }
    }
}

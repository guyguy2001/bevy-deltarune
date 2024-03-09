use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use bevy_rapier2d::prelude::*;

use crate::AppState;

use super::healthbar::Health;
use super::player::Player;

pub struct BulletsPlugin;

impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            player_collision.run_if(in_state(AppState::Defending)),
        );
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
struct Bullet {
    pub direction: Vec3,
    pub damage: f32,
}

pub struct BulletProperties {
    pub damage: f32,
    pub size: f32,
    pub speed: f32,
}

pub fn spawn_bullet_in_pos(position: Vec3, direction: Vec3, properties: BulletProperties, commands: &mut Commands) {
    // TODO: Question - when do I receive asset_server as a parameter, and when do I get it from the world?
    // TODO: ask for the asset_server inside of the commands queue, instead of directly here?
    commands.add(move |world: &mut World| {
        let asset_server = world.get_resource::<AssetServer>().unwrap(); // TODO: How do I not unwrap?
        world.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(properties.size, properties.size)),
                    ..default()
                },
                texture: asset_server.load("pig.png"),
                transform: Transform {
                    translation: position,
                    ..Default::default()
                },
                ..Default::default()
            },
            Bullet {
                direction: direction,
                damage: properties.damage,
            },
            (
                ActiveEvents::COLLISION_EVENTS,
                ActiveCollisionTypes::all(),
                RigidBody::KinematicVelocityBased,
                Velocity {
                    linvel: direction.xy().normalize() * properties.speed,
                    ..Default::default()
                },
                Collider::cuboid(properties.size / 2.0, properties.size / 2.0),
                Sensor,
            ),
        ));
    });
}

fn player_collision(
    mut commands: Commands,
    mut contact_events: EventReader<CollisionEvent>,
    bullets: Query<(Entity, &Bullet)>,
    mut players: Query<&mut Health, With<Player>>,
) {
    for event in contact_events.read() {
        // println!("{:?}", event);
        if let CollisionEvent::Started(entity1, entity2, _) = event {
            // TODO: get this working with swapped entity orders???
            if let Ok(mut player_health) = players.get_mut(*entity1) {
                if let Ok((bullet_entity, bullet_component)) = bullets.get(*entity2) {
                    commands.entity(bullet_entity).despawn_recursive();

                    // TODO: make this an event? Who is responsible for handling it? what would it achieve?
                    player_health.health -= bullet_component.damage;
                }
            }
        }
    }
}

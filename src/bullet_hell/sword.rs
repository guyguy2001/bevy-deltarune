use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_inspector_egui::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tween::tween::TargetAsset;
use bevy_tween::{combinator::*, prelude::*, tween_event_system};

use super::bullet::Bullet;
use super::physics_layers;

pub struct SwordPlugin;

impl Plugin for SwordPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultTweenPlugins)
            .add_event::<TweenEvent<SwordAnimationEvent>>()
            .add_systems(
                Update,
                (
                    sword_animation_events,
                    tween_event_system::<SwordAnimationEvent>,
                ),
            )
            .add_systems(FixedUpdate, bullet_collision);
    }
}

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
struct SwordAttack {
    lifetime: Timer,
}

impl SwordAttack {
    pub fn new(lifetime: Duration) -> Self {
        Self {
            lifetime: Timer::new(lifetime, TimerMode::Once),
        }
    }
}

impl Default for SwordAttack {
    fn default() -> Self {
        Self::new(Duration::from_secs_f32(0.5))
    }
}

#[derive(Clone)]
enum SwordAnimationEvent {
    Finished,
}

fn secs(secs: f32) -> Duration {
    Duration::from_secs_f32(secs)
}

pub fn spawn_sword(
    parent: Entity,
    // TODO: Role, take from laser's code
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let starting_color = Color::WHITE.with_a(0.);

    let color_material = materials.add(starting_color);

    let mut color_animation: TargetState<TargetAsset<ColorMaterial>, Color> =
        color_material.clone().into_target().state(starting_color);

    let size = 15.;

    commands.entity(parent).with_children(|builder| {
        // TODO: Improve the animation, make the sword only active for part of the animation
        builder
            .spawn((
                SwordAttack::default(),
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(size)).into(),
                    material: color_material,
                    transform: Transform::from_translation(-Vec3::Z),
                    ..Default::default()
                },
                Name::new("Sword animation"),
                (
                    ActiveEvents::COLLISION_EVENTS,
                    ActiveCollisionTypes::all(),
                    Collider::ball(size),
                    CollisionGroups::new(physics_layers::PLAYER_BULLETS, physics_layers::ALL),
                    RigidBody::KinematicPositionBased,
                    Sensor,
                ),
            ))
            .animation()
            .insert(sequence((
                tween(
                    secs(0.1),
                    EaseFunction::CircularOut,
                    color_animation
                        .with(interpolate::color_material_to(starting_color.with_a(1.0))),
                ),
                tween(
                    secs(0.1),
                    EaseFunction::CircularIn,
                    color_animation
                        .with(interpolate::color_material_to(starting_color.with_a(0.0))),
                ),
                event(SwordAnimationEvent::Finished),
            )));
    });
}

fn sword_animation_events(
    mut event: EventReader<TweenEvent<SwordAnimationEvent>>,
    parent_query: Query<&Parent>,
    mut commands: Commands,
) {
    event.read().for_each(|event| match event.data {
        SwordAnimationEvent::Finished => {
            let sword_entity = parent_query.get(event.entity).unwrap();
            commands.entity(sword_entity.get()).despawn_recursive();
        }
    });
}

fn bullet_collision(
    mut commands: Commands,
    mut contact_events: EventReader<CollisionEvent>,
    bullets: Query<Entity, With<Bullet>>,
    swords: Query<Entity, With<SwordAttack>>,
) {
    // TODO: How do I describe which entities will be destroyed by this? Just anything hittable and hostile?
    for event in contact_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = event {
            // TODO: get this working with swapped entity orders???
            if swords.contains(*entity1) {
                if let Ok(bullet_entity) = bullets.get(*entity2) {
                    commands.entity(bullet_entity).despawn_recursive();
                }
            }
            if swords.contains(*entity2) {
                if let Ok(bullet_entity) = bullets.get(*entity1) {
                    commands.entity(bullet_entity).despawn_recursive();
                }
            }
        }
    }
}

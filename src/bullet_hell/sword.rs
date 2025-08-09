use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_tween::tween::TargetAsset;
use bevy_tween::{combinator::*, prelude::*, tween_event_system};

use crate::bullet_hell::bullet::Bullet;
use crate::bullet_hell::physics_layers;

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
    parent: In<Entity>,
    // TODO: Role, take from laser's code
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let starting_color = Color::WHITE.with_alpha(0.);

    let color_material = materials.add(starting_color);

    let mut color_animation: TargetState<TargetAsset<ColorMaterial>, Color> =
        color_material.clone().into_target().state(starting_color);

    let size = 15.;

    commands.entity(*parent).with_children(|builder| {
        // TODO: Improve the animation, make the sword only active for part of the animation
        builder
            .spawn((
                SwordAttack::default(),
                Mesh2d(meshes.add(Circle::new(size))),
                MeshMaterial2d(color_material),
                Transform::from_translation(-Vec3::Z),
                Name::new("Sword animation"),
                (
                    CollisionLayers::new(
                        physics_layers::GameLayers::PlayerBullet,
                        physics_layers::GameLayers::Bullet,
                    ),
                    RigidBody::Kinematic,
                    Collider::circle(size),
                    Sensor,
                    CollisionEventsEnabled,
                ),
            ))
            .animation()
            .insert(sequence((
                tween(
                    secs(0.1),
                    EaseKind::CircularOut,
                    color_animation.with(interpolate::color_material_to(
                        starting_color.with_alpha(1.0),
                    )),
                ),
                tween(
                    secs(0.1),
                    EaseKind::CircularIn,
                    color_animation.with(interpolate::color_material_to(
                        starting_color.with_alpha(0.0),
                    )),
                ),
                event(SwordAnimationEvent::Finished),
            )));
    });
}

fn sword_animation_events(
    mut event: EventReader<TweenEvent<SwordAnimationEvent>>,
    parent_query: Query<&ChildOf>,
    mut commands: Commands,
) {
    event.read().for_each(|event| match event.data {
        SwordAnimationEvent::Finished => {
            let sword_entity = parent_query.get(event.entity).unwrap();
            commands.entity(sword_entity.parent()).despawn();
        }
    });
}

fn bullet_collision(
    mut commands: Commands,
    mut contact_events: EventReader<CollisionStarted>,
    names: Query<&Name>,
    bullets: Query<Entity, With<Bullet>>,
    swords: Query<Has<SwordAttack>>,
) -> Result {
    // TODO: How do I describe which entities will be destroyed by this? Just anything hittable and hostile?
    for event in contact_events.read() {
        let CollisionStarted(entity1, entity2) = event;
        // TODO: get this working with swapped entity orders???
        let names = [
            names
                .get(*entity1)
                .map_or(format!("{entity1}"), ToString::to_string),
            names
                .get(*entity2)
                .map_or(format!("{entity2}"), ToString::to_string),
        ];
        println!("Sword collision {names:?}");
        if swords.get(*entity1)? {
            if let Ok(bullet_entity) = bullets.get(*entity2) {
                commands.entity(bullet_entity).despawn();
            }
        }
        if swords.get(*entity2)? {
            if let Ok(bullet_entity) = bullets.get(*entity1) {
                commands.entity(bullet_entity).despawn();
            }
        }
    }
    Ok(())
}

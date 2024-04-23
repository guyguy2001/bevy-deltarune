use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    bullet_hell::{game_z_index, health::TryDamageEvent, player::Player},
    AppState,
};

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_initial_laser_cannons)
            .add_systems(
                Update,
                (
                    laser_cannon_behavior,
                    laser_lifecycle,
                    laser_player_collision,
                )
                    .run_if(in_state(AppState::Defending)),
            );
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
struct LaserCannon {
    // TODO: Timer duplication with moving_cannon
    // Do I want to have a component with a one-shot callback for when the timer is done?
    // Maybe have a component with just the timer, and a system to check whenever it's done?
    // So for (repeated_attacks, laser) in query { if repeated_attacks.just_finished {laser.shoot()}}
    shooting_timer: Timer,
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
struct Shooting {}

fn laser_cannon_behavior(
    mut query: Query<(Entity, &mut LaserCannon)>,
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, mut laser_cannon) in query.iter_mut() {
        laser_cannon.shooting_timer.tick(time.delta());
        if laser_cannon.shooting_timer.just_finished() {
            spawn_laser(entity, &mut commands, &mut meshes, &mut materials);
        }
    }
}

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
struct Laser {
    // TODO: probably a curve and not just a duration
    winding_up_duration: Duration,
    active_duration: Duration,
    winding_down_duration: Duration,
    damage: f32,
}

impl Default for Laser {
    fn default() -> Self {
        Self {
            winding_up_duration: Duration::from_secs(1),
            active_duration: Duration::from_secs(2),
            winding_down_duration: Duration::from_secs_f32(0.25),
            damage: 5.,
        }
    }
}

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
enum LaserState {
    WindingUp(Timer),
    Active(Timer),
    WindingDown(Timer),
}

impl Default for LaserState {
    // TODO: I don't want this to have a default. I think I have to do this for the inspector?
    fn default() -> Self {
        Self::WindingUp(Timer::from_seconds(20., TimerMode::Once))
    }
}

impl LaserState {
    fn new(laser: &Laser) -> Self {
        Self::make_winding_up(laser)
    }

    fn make_winding_up(laser: &Laser) -> Self {
        Self::WindingUp(Timer::new(laser.winding_up_duration, TimerMode::Once))
    }

    fn make_active(laser: &Laser) -> Self {
        Self::Active(Timer::new(laser.active_duration, TimerMode::Once))
    }

    fn make_winding_down(laser: &Laser) -> Self {
        Self::WindingDown(Timer::new(laser.winding_down_duration, TimerMode::Once))
    }
}

fn spawn_laser(
    parent: Entity,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    const WIDTH: f32 = 10.;
    const HEIGHT: f32 = 1000.;
    let laser = Laser::default();
    let child = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::new(WIDTH, HEIGHT)).into(), // TODO: custom width, full height (TODO: rotations),
                material: materials.add(Color::WHITE.with_a(0.)),
                transform: Transform::from_xyz(0., -HEIGHT / 2., game_z_index::LASERS),
                ..Default::default()
            },
            LaserState::new(&laser),
            laser,
            Name::new("Laser"),
            (
                ActiveEvents::COLLISION_EVENTS,
                ActiveCollisionTypes::all(),
                ColliderDisabled,
                Collider::cuboid(WIDTH / 2., HEIGHT / 2.),
                RigidBody::Fixed,
                Sensor,
            ),
        ))
        .id();
    commands.entity(parent).add_child(child);
}

fn laser_lifecycle(
    mut query: Query<(Entity, &Laser, &mut LaserState, &mut Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, laser, mut state, material_handle) in query.iter_mut() {
        let mut lerp_color = |start: f32, end: f32, timer_fraction| {
            materials
                .get_mut(material_handle.id())
                .unwrap()
                .color
                .set_a((start).lerp(end, timer_fraction));
        };

        match state.as_mut() {
            LaserState::WindingUp(ref mut timer) => {
                timer.tick(time.delta());
                lerp_color(0., 1., timer.fraction());

                if timer.just_finished() {
                    commands.entity(entity).remove::<ColliderDisabled>();
                    *state = LaserState::make_active(laser);
                }
            }
            LaserState::Active(ref mut timer) => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    commands.entity(entity).insert(ColliderDisabled);
                    *state = LaserState::make_winding_down(laser);
                }
            }
            LaserState::WindingDown(ref mut timer) => {
                timer.tick(time.delta());
                lerp_color(1., 0., timer.fraction());

                if timer.just_finished() {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}

fn laser_player_collision(
    mut contact_events: EventReader<CollisionEvent>,
    q_lasers: Query<&Laser>,
    q_players: Query<Entity, With<Player>>,
    mut damage_writer: EventWriter<TryDamageEvent>,
) {
    for event in contact_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = event {
            // TODO: get this working with swapped entity orders???
            if let Ok(player_entity) = q_players.get(*entity1) {
                if let Ok(laser_component) = q_lasers.get(*entity2) {
                    damage_writer.send(TryDamageEvent {
                        damage: laser_component.damage,
                        target_entity: player_entity,
                    });
                }
            }
        }
    }
}

fn spawn_initial_laser_cannons(mut commands: Commands, asset_server: Res<AssetServer>) {
    let position = Vec3::new(0., 70., game_z_index::CANNONS);
    let sprite_size = 7.5;
    let texture = asset_server.load("character.png");
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(sprite_size, sprite_size)),
                ..default()
            },
            texture: texture.clone(),
            transform: Transform::from_translation(position),
            ..default()
        },
        LaserCannon {
            shooting_timer: Timer::from_seconds(1., TimerMode::Once),
        },
        Name::new("Laser Cannon"),
    ));
}

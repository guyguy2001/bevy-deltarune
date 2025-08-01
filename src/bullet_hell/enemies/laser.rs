use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
// use bevy_rapier2d::prelude::*;

use crate::{
    bullet_hell::{game_z_index, health::TryDamageEvent, physics_layers, player::Player},
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
            )
            .register_type::<LaserCannon>()
            .register_type::<Laser>()
            .register_type::<LaserState>();
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
    has_active_laser: bool,
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
        if !laser_cannon.has_active_laser {
            laser_cannon.shooting_timer.tick(time.delta());
            if laser_cannon.shooting_timer.just_finished() {
                spawn_laser(entity, &mut commands, &mut meshes, &mut materials);
                laser_cannon.has_active_laser = true
            }
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
            Mesh2d(meshes.add(Rectangle::new(WIDTH, HEIGHT))), // TODO: custom width, full height (TODO: rotations),
            MeshMaterial2d(materials.add(Color::WHITE.with_alpha(0.))),
            Transform::from_xyz(0., -HEIGHT / 2., game_z_index::LASERS),
            LaserState::new(&laser),
            laser,
            Name::new("Laser"),
            (
                CollisionLayers::new(
                    physics_layers::GameLayers::Bullet,
                    physics_layers::GameLayers::all_bits(),
                ),
                RigidBody::Static,
                Collider::rectangle(WIDTH, HEIGHT),
                Sensor,
            ),
        ))
        .id();
    commands.entity(parent).add_child(child);
}

fn laser_lifecycle(
    mut cannon_query: Query<&mut LaserCannon>,
    mut laser_query: Query<(
        Entity,
        &ChildOf,
        &Laser,
        &mut LaserState,
        &mut MeshMaterial2d<ColorMaterial>,
    )>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, child_of, laser, mut state, material_handle) in laser_query.iter_mut() {
        let mut lerp_color = |start: f32, end: f32, timer_fraction| {
            materials
                .get_mut(material_handle.id())
                .unwrap()
                .color
                .set_alpha((start).lerp(end, timer_fraction));
        };

        match state.as_mut() {
            LaserState::WindingUp(ref mut timer) => {
                timer.tick(time.delta());
                lerp_color(0., 1., timer.fraction());

                if timer.just_finished() {
                    // commands.entity(entity).remove::<ColliderDisabled>();
                    *state = LaserState::make_active(laser);
                }
            }
            LaserState::Active(ref mut timer) => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    // commands.entity(entity).insert(ColliderDisabled);
                    *state = LaserState::make_winding_down(laser);
                }
            }
            LaserState::WindingDown(ref mut timer) => {
                timer.tick(time.delta());
                lerp_color(1., 0., timer.fraction());

                if timer.just_finished() {
                    commands.entity(entity).despawn();
                }
                cannon_query
                    .get_mut(child_of.parent())
                    .unwrap()
                    .has_active_laser = false;
            }
        }
    }
}

fn laser_player_collision(
    mut contact_events: EventReader<CollisionStarted>,
    lasers: Query<&Laser>,
    players: Query<Entity, With<Player>>,
    mut damage_writer: EventWriter<TryDamageEvent>,
) {
    for event in contact_events.read() {
        let CollisionStarted(entity1, entity2) = event;
        // TODO: get this working with swapped entity orders???
        println!("Collision: {entity1} {entity2}");
        let (player_entity, bullet_component) =
            if let (Ok(player), Ok(enemy)) = (players.get(*entity1), lasers.get(*entity2)) {
                (player, enemy)
            } else if let (Ok(player), Ok(enemy)) = (players.get(*entity2), lasers.get(*entity1)) {
                (player, enemy)
            } else {
                return;
            };

        // TODO: make this an event? Who is responsible for handling it? what would it achieve?
        damage_writer.write(TryDamageEvent {
            target_entity: player_entity,
            damage: bullet_component.damage,
        });
    }
}

fn spawn_initial_laser_cannons(mut commands: Commands, asset_server: Res<AssetServer>) {
    let position = Vec3::new(0., 70., game_z_index::CANNONS);
    let sprite_size = 7.5;
    let texture = asset_server.load("character.png");
    commands.spawn((
        Transform::from_translation(position),
        Sprite {
            custom_size: Some(Vec2::new(sprite_size, sprite_size)),
            ..Sprite::from_image(texture)
        },
        LaserCannon {
            shooting_timer: Timer::from_seconds(1., TimerMode::Repeating),
            has_active_laser: false,
        },
        Name::new("Laser Cannon"),
    ));
}

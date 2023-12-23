use std::time::Duration;

use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::bullet_hell::bullet::spawn_bullet_in_pos;
use crate::AppState;

pub struct MovingCannonPlugin;

impl Plugin for MovingCannonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cannons).add_systems(
            Update,
            (cannon_behaviour, wander_behaviour).run_if(in_state(AppState::Defending)),
        );
    }
}

#[derive(Clone, Copy, Reflect, Default)]
enum EnemyTypes {
    #[default]
    Bullet,
}

#[derive(Component, InspectorOptions, Reflect, Default)]
#[reflect(Component, InspectorOptions)]
struct Cannon {
    shooting_timer: Timer,
    direction: Vec3,
    spawned_enemy_type: EnemyTypes,
}

impl Cannon {
    pub fn new(enemy_type: EnemyTypes, direction: Vec3, duration: Duration) -> Self {
        Self {
            shooting_timer: Timer::new(duration, TimerMode::Repeating),
            direction,
            spawned_enemy_type: enemy_type,
        }
    }
}

fn spawn_enemy(cannon: &Cannon, position: Vec3, enemy_type: EnemyTypes, commands: &mut Commands) {
    match enemy_type {
        EnemyTypes::Bullet => spawn_bullet_in_pos(position, cannon.direction, commands),
    };
}

fn cannon_behaviour(
    time: Res<Time>,
    mut commands: Commands,
    mut cannons: Query<(&mut Cannon, &Transform)>,
) {
    for (mut cannon, transform) in cannons.iter_mut() {
        cannon.shooting_timer.tick(time.delta());
        if cannon.shooting_timer.just_finished() {
            // TODO: Global translation
            spawn_enemy(
                &cannon,
                transform.translation,
                cannon.spawned_enemy_type,
                &mut commands,
            );
        }
    }
}

#[derive(Reflect, Default)]
enum WanderDirection {
    #[default]
    ToEnd,
    ToStart,
}

impl WanderDirection {
    pub fn other(&self) -> Self {
        match self {
            &Self::ToEnd => Self::ToStart,
            &Self::ToStart => Self::ToEnd,
        }
    }
}

struct CannonSpawnProperties {
    start: Vec3,
    end: Vec3,
    direction: Vec3,
}

#[derive(Component, InspectorOptions, Reflect, Default)]
#[reflect(Component, InspectorOptions)]
struct Wander {
    start: Vec3,
    end: Vec3,
    // Todo: replace speed with Duration / Timer to allow finer control of wandering vs shooting (right now we shoot 1.999 times per wander)
    speed: f32,
    direction: WanderDirection,
}

fn wander_behaviour(time: Res<Time>, mut wander_query: Query<(&mut Wander, &mut Transform)>) {
    for (mut wander, mut transform) in wander_query.iter_mut() {
        let destination = match wander.direction {
            WanderDirection::ToEnd => wander.end,
            WanderDirection::ToStart => wander.start,
        };
        let translation_left = destination - transform.translation;
        let desired_movement =
            translation_left.normalize_or_zero() * wander.speed * time.delta().as_secs_f32();
        if desired_movement.length_squared() > translation_left.length_squared() {
            transform.translation = destination;
            wander.direction = wander.direction.other();
        } else {
            transform.translation += desired_movement;
        }
    }
}

fn spawn_cannons(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("character.png");
    let sprite_size = 7.5;
    let cannons = [
        CannonSpawnProperties {
            start: Vec3::new(-100., -40., 0.),
            end: Vec3::new(-100., 40., 0.),
            direction: Vec3::X,
        },
        CannonSpawnProperties {
            start: Vec3::new(-40., 70., 0.),
            end: Vec3::new(40., 70., 0.),
            direction: Vec3::NEG_Y,
        },
    ];
    for cannon in cannons {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(sprite_size, sprite_size)),
                    ..default()
                },
                texture: texture.clone(),
                transform: Transform::from_translation(cannon.start),
                ..default()
            },
            Cannon::new(EnemyTypes::Bullet, cannon.direction, Duration::from_secs(1)),
            Wander {
                start: cannon.start,
                end: cannon.end,
                speed: 50.,
                ..Default::default()
            },
        ));
    }
}

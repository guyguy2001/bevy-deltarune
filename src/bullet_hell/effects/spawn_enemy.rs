use std::{f32::consts::{PI, TAU}, time::Duration};

use bevy::prelude::*;
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};
use rand::Rng;

use crate::bullet_hell::enemies::moving_cannon::{
    spawn_cannon, spawn_stationary_cannon, CannonSpawnProperties,
};

use super::{effect::{Effect, LevelTransitionEffectsPool}, spawning_animation::SpawningAnimation};

pub struct SpawnEnemyPlugin;

impl Plugin for SpawnEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, register_effects);
    }
}

pub fn register_effects(world: &mut World) {
    // IDEA: I can have this be a system with an Output of Vec<Effect>, and have the registration happen in a centralized place
    let effects = [
        Effect(world.register_system(spawn_cannon_on_the_right)),
        Effect(world.register_system(spawn_random_stationary_cannon)),
    ];
    for e in effects {
        let mut effects_res = world
            .get_resource_mut::<LevelTransitionEffectsPool>()
            .unwrap();
        effects_res.add_effect(e);
    }
}

pub fn spawn_cannon_on_the_right(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_cannon(
        CannonSpawnProperties {
            start: Vec3::new(100., -40., 0.),
            end: Vec3::new(100., 40., 0.),
            shooting_direction: -Vec3::X,
        },
        SpawningAnimation::new(Duration::from_millis(500)),
        &asset_server,
        &mut commands,
    )
}

pub fn spawn_random_stationary_cannon(mut commands: Commands, asset_server: Res<AssetServer>) {
    let options = [
        (Vec3::new(100., 70., 0.), Vec3::new(-1., -1., 0.)),
        (Vec3::new(-100., 70., 0.), Vec3::new(1., -1., 0.)),
        (Vec3::new(100., -70., 0.), Vec3::new(-1., 1., 0.)),
        (Vec3::new(-100., -70., 0.), Vec3::new(1., 1., 0.)),
    ];
    let selected = options.get(rand::thread_rng().gen_range(0..4)).unwrap();
    spawn_stationary_cannon(
        CannonSpawnProperties {
            start: selected.0,
            end: Vec3::ZERO,
            shooting_direction: selected.1,
        },
        SpawningAnimation::new(Duration::from_millis(500)),
        &asset_server,
        &mut commands,
    )
}

use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_inspector_egui::prelude::*;
use bevy_tween::tween::TargetAsset;
use bevy_tween::{combinator::*, prelude::*};

pub struct SwordPlugin;

impl Plugin for SwordPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultTweenPlugins);
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
            )));
    });
}

use std::path::Path;
use std::time::Duration;

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_tween::{combinator::*, prelude::*, tween::AnimationTarget};

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
    asset_server: ResMut<AssetServer>,
    mut commands: Commands,
) {
    let triangle = AnimationTarget.into_target();

    let starting_color = Color::WHITE.with_a(0.);
    let mut color_animation = triangle.state(starting_color);
    let size = 15.;

    commands.entity(parent).with_children(|builder| {
        // TODO: Improve the animation, make the sword only active for part of the animation
        builder
            .spawn((
                SwordAttack::default(),
                SpriteBundle {
                    texture: asset_server.load(Path::new("sprites/circle.png")).clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(size, size)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                AnimationTarget,
                Name::new("Sword animation"),
            ))
            .animation()
            .insert(sequence((
                tween(
                    secs(0.1),
                    EaseFunction::CircularOut,
                    color_animation.with(interpolate::sprite_color_to(starting_color.with_a(1.0))),
                ),
                tween(
                    secs(0.1),
                    EaseFunction::CircularIn,
                    color_animation.with(interpolate::sprite_color_to(starting_color.with_a(0.0))),
                ),
            )));
    });
}

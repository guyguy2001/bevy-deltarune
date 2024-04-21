use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::ColorMaterialColorLens, *};

use super::health::OnDamageDealt;

pub struct HitEffectPlugin;

impl Plugin for HitEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, flash_on_damage);
    }
}

fn flash_on_damage(
    mut reader: EventReader<OnDamageDealt>,
    mut color_query: Query<&Handle<ColorMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    // TODO: Do I want to make this work for sprites as well, via an overlay sprite or something?
    for event in reader.read() {
        let color_handle = color_query.get_mut(event.target_entity).unwrap();
        let material = materials.get_mut(color_handle).unwrap();
        let current_color = material.color;
        material.color = Color::RED;
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs_f32(0.2),
            ColorMaterialColorLens {
                start: Color::RED,
                end: current_color,
            },
        );
        commands
            .entity(event.target_entity)
            .insert(AssetAnimator::new(tween));
    }
}

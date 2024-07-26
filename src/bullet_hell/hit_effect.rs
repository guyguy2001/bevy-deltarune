use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::*;

use crate::utils::animation::lenses::ColorMaterialRGBLens;

use super::health::{DamageOccurredEvent, Invulnerability};

pub struct HitEffectPlugin;

impl Plugin for HitEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                flash_on_damage,
                add_invulnerability_effect,
                remove_invulnerability_effect,
            ),
        );
    }
}

// TODO: This animation happens even when there's the invul component
fn flash_on_damage(
    mut reader: EventReader<DamageOccurredEvent>,
    mut color_query: Query<&Handle<ColorMaterial>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    // TODO: Do I want to make this work for sprites as well, via an overlay sprite or something?
    // I definately want this to be overlayed, as currently it is affected by the alpha - as the red
    // fades out I want to reveal an already opacitied-out player.

    for event in reader.read() {
        let color_handle = color_query.get_mut(event.target_entity).unwrap();
        let material = materials.get(color_handle).unwrap();
        let current_color = material.color;
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs_f32(0.2),
            // I'm using this and not the standard ColorLens to allow
            // messing with the alpha in the invulnerability systems.
            ColorMaterialRGBLens {
                start: bevy::color::palettes::basic::RED.into(),
                end: current_color,
            },
        );
        commands
            .entity(event.target_entity)
            .insert(AssetAnimator::new(tween));
    }
}

fn add_invulnerability_effect(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<&Handle<ColorMaterial>, Added<Invulnerability>>,
) {
    for color_handle in query.iter_mut() {
        materials
            .get_mut(color_handle)
            .unwrap()
            .color
            .set_alpha(0.25);
    }
}

fn remove_invulnerability_effect(
    mut removed_components: RemovedComponents<Invulnerability>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&Handle<ColorMaterial>>,
) {
    for entity in removed_components.read() {
        let color_handle = query.get(entity).unwrap();
        materials.get_mut(color_handle).unwrap().color.set_alpha(1.);
    }
}

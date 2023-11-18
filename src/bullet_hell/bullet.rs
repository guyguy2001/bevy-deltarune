use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::AppState;

pub struct BulletsPlugin;

impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bullets)
            .add_systems(Update, move_bullets.run_if(in_state(AppState::Defending)));
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
struct Bullet {
    pub direction: Vec3,
}

fn spawn_bullets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pig.png"),
            transform: Transform {
                translation: Vec3::new(-50.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        Bullet {
            direction: Vec3::new(200.0, 0.0, 0.0),
        },
    ));
}

fn move_bullets(mut bullets: Query<(&Bullet, &mut Transform)>, time: Res<Time>) {
    for (bullet, mut transform) in bullets.iter_mut() {
        transform.translation += bullet.direction * time.delta_seconds();
    }
}

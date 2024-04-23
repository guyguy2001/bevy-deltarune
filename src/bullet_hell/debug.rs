use bevy::prelude::*;

use super::{health::TryDamageEvent, player::Player};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_damage);
    }
}

fn apply_damage(
    player: Query<Entity, With<Player>>,
    mut damage_writer: EventWriter<TryDamageEvent>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        damage_writer.send(TryDamageEvent {
            target_entity: player.single(),
            damage: 5.,
        });
    }
}

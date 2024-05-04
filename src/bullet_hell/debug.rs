use bevy::prelude::*;

use super::{health::TryDamageEvent, level::CombatFinishedEvent, player::Player};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (apply_damage, skip_level));
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

fn skip_level(mut win_event: EventWriter<CombatFinishedEvent>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::KeyG) {
        win_event.send(CombatFinishedEvent);
    }
}

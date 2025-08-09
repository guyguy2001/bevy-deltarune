use bevy::prelude::*;

use super::{
    health::{Health, TryDamageEvent},
    level::CombatFinishedEvent,
    player::Player,
    sword::spawn_sword,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                apply_damage,
                skip_level,
                debug_spawn_sword,
                make_player_invincible,
            ),
        );
    }
}

fn apply_damage(
    player: Query<Entity, With<Player>>,
    mut damage_writer: EventWriter<TryDamageEvent>,
    input: Res<ButtonInput<KeyCode>>,
) -> Result {
    if input.just_pressed(KeyCode::KeyR) {
        damage_writer.write(TryDamageEvent {
            target_entity: player.single()?,
            damage: 5.,
        });
    }
    Ok(())
}

fn skip_level(mut win_event: EventWriter<CombatFinishedEvent>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::KeyG) {
        win_event.write(CombatFinishedEvent);
    }
}

fn debug_spawn_sword(
    input: Res<ButtonInput<KeyCode>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    commands: Commands,
    q_player: Query<Entity, With<Player>>,
) {
    if input.just_pressed(KeyCode::KeyP) {
        spawn_sword(In(q_player.single().unwrap()), meshes, materials, commands);
    }
}

fn make_player_invincible(
    mut player: Query<&mut Health, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) -> Result {
    if input.just_pressed(KeyCode::KeyR) {
        player.single_mut()?.max_health += 10000.;
        player.single_mut()?.health += 10000.;
    }

    Ok(())
}

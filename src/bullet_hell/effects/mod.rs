use bevy::prelude::*;

use self::effect::LevelTransitionEffectsPool;

pub mod effect;
mod spawn_enemy;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelTransitionEffectsPool>()
            .add_plugins(spawn_enemy::SpawnEnemyPlugin);
    }
}

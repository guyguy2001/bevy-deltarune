use bevy::prelude::*;

use self::effect::LevelTransitionEffectsPool;

pub mod effect;
pub mod spawning_animation;
mod spawn_enemy;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelTransitionEffectsPool>()
            .add_plugins(spawn_enemy::SpawnEnemyPlugin)
            .add_plugins(spawning_animation::SpawningAnimationPlugin);
    }
}

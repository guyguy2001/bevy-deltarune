use bevy::prelude::*;

pub use self::level::{CurrentLevelConfig, LevelConfig, LevelFinishedEvent};

mod effects;
mod enemies;
mod game_ui;
mod physics_layers;

mod arena;
mod bullet;
mod dash;
mod debug;
mod game_z_index;
mod health;
mod hit_effect;
mod level;
mod level_end_animation;
mod level_timer;
mod player;
mod sword;
mod upgrades;

pub struct BulletHellPlugin;

impl Plugin for BulletHellPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            arena::ArenaPlugin,
            bullet::BulletsPlugin,
            dash::DashPlugin,
            debug::DebugPlugin,
            effects::EffectsPlugin,
            enemies::laser::LaserPlugin,
            enemies::moving_cannon::MovingCannonPlugin,
            game_ui::GameUIPlugin,
            health::HealthPlugin,
            hit_effect::HitEffectPlugin,
            level::LevelPlugin,
            level_end_animation::LevelEndAnimationPlugin,
            level_timer::LevelTimerPlugin,
            player::PlayerPlugin,
            sword::SwordPlugin,
        ))
        .add_systems(Startup, upgrades::populate_upgrades_pool);
    }
}

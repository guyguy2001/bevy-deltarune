use bevy::prelude::*;

use self::arena::ArenaPlugin;
use self::bullet::BulletsPlugin;
use self::dash::DashPlugin;
use self::debug::DebugPlugin;
use self::effects::EffectsPlugin;
use self::enemies::laser::LaserPlugin;
use self::enemies::moving_cannon::MovingCannonPlugin;
use self::game_ui::GameUIPlugin;
use self::health::HealthPlugin;
use self::hit_effect::HitEffectPlugin;
use self::level::LevelPlugin;
use self::level_end_animation::LevelEndAnimationPlugin;
use self::level_timer::LevelTimerPlugin;
use self::player::PlayerPlugin;
use self::sword::SwordPlugin;
use self::upgrades::populate_upgrades_pool;

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
            ArenaPlugin,
            BulletsPlugin,
            DashPlugin,
            DebugPlugin,
            EffectsPlugin,
            GameUIPlugin,
            HealthPlugin,
            HitEffectPlugin,
            LaserPlugin,
            LevelEndAnimationPlugin,
            LevelTimerPlugin,
            LevelPlugin,
            MovingCannonPlugin,
            // PhysicsPlugin,
            PlayerPlugin,
            SwordPlugin,
        ))
        .add_systems(Startup, populate_upgrades_pool);
    }
}

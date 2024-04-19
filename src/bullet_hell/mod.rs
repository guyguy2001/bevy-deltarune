use bevy::prelude::*;

use self::arena::ArenaPlugin;
use self::bullet::BulletsPlugin;
use self::effects::EffectsPlugin;
use self::enemies::laser::LaserPlugin;
use self::enemies::moving_cannon::MovingCannonPlugin;
use self::healthbar::HealthbarPlugin;
use self::level::LevelPlugin;
use self::level_end_animation::LevelEndAnimationPlugin;
use self::level_timer::LevelTimerPlugin;
use self::player::PlayerPlugin;
use self::upgrades::populate_upgrades_pool;

mod effects;
mod enemies;

mod arena;
mod bullet;
mod game_z_index;
mod healthbar;
mod level;
mod level_end_animation;
mod level_timer;
mod physics;
mod player;
mod upgrades;

pub struct BulletHellPlugin;

impl Plugin for BulletHellPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ArenaPlugin,
            BulletsPlugin,
            EffectsPlugin,
            HealthbarPlugin,
            LaserPlugin,
            LevelEndAnimationPlugin,
            LevelTimerPlugin,
            LevelPlugin,
            MovingCannonPlugin,
            // PhysicsPlugin,
            PlayerPlugin,
        ))
        .add_systems(Startup, populate_upgrades_pool);
    }
}

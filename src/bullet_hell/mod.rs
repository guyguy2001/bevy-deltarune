use bevy::prelude::*;

use self::arena::ArenaPlugin;
use self::bullet::BulletsPlugin;
use self::effects::EffectsPlugin;
use self::enemies::moving_cannon::MovingCannonPlugin;
use self::healthbar::HealthbarPlugin;
use self::level::LevelPlugin;
use self::level_end_animation::LevelEndAnimationPlugin;
use self::level_timer::LevelTimerPlugin;
use self::player::PlayerPlugin;

mod effects;
mod enemies;

mod arena;
mod bullet;
mod healthbar;
mod level;
mod level_end_animation;
mod level_timer;
mod physics;
mod player;

pub struct BulletHellPlugin;

impl Plugin for BulletHellPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ArenaPlugin,
            BulletsPlugin,
            EffectsPlugin,
            HealthbarPlugin,
            LevelEndAnimationPlugin,
            LevelTimerPlugin,
            LevelPlugin,
            MovingCannonPlugin,
            // PhysicsPlugin,
            PlayerPlugin,
        ));
    }
}

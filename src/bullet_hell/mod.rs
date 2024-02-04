use bevy::prelude::*;

use self::arena::ArenaPlugin;
use self::bullet::BulletsPlugin;
use self::enemies::moving_cannon::MovingCannonPlugin;
use self::healthbar::HealthbarPlugin;
use self::level::LevelPlugin;
use self::level_timer::LevelTimerPlugin;
use self::player::PlayerPlugin;

mod enemies;

mod arena;
mod bullet;
mod healthbar;
mod level;
mod level_timer;
mod player;

pub struct BulletHellPlugin;

impl Plugin for BulletHellPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ArenaPlugin,
            BulletsPlugin,
            HealthbarPlugin,
            LevelTimerPlugin,
            LevelPlugin,
            MovingCannonPlugin,
            PlayerPlugin,
        ));
    }
}

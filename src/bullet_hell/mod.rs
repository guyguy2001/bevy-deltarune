use bevy::prelude::*;

use arena::ArenaPlugin;
use bullet::BulletsPlugin;
use enemies::moving_cannon::MovingCannonPlugin;
use player::PlayerPlugin;

mod enemies;

mod arena;
mod bullet;
mod healthbar;
mod player;

pub struct BulletHellPlugin;

impl Plugin for BulletHellPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MovingCannonPlugin, BulletsPlugin, PlayerPlugin, ArenaPlugin));
    }
}

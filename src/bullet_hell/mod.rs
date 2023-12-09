use bevy::prelude::*;

use arena::ArenaPlugin;
use bullet::BulletsPlugin;
use player::PlayerPlugin;

mod arena;
mod bullet;
mod player;

pub struct BulletHellPlugin;

impl Plugin for BulletHellPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BulletsPlugin, PlayerPlugin, ArenaPlugin));
    }
}

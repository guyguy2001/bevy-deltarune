use bevy::prelude::*;

mod ability_lib;
mod game_abilities;
mod parry;

pub use ability_lib::AbilityUpgradePool;

pub fn plugin(app: &mut App) {
    app.add_plugins((ability_lib::plugin, game_abilities::plugin));
}

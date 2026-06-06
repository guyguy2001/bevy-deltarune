use bevy::prelude::*;

use self::healthbar::HealthbarPlugin;

pub mod abilities;
pub mod healthbar;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((abilities::plugin, HealthbarPlugin));
    }
}

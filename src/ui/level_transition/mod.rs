use bevy::prelude::*;

use self::menu::LevelTransitionMenuPlugin;

mod menu;

pub struct LevelTransitionPlugin;

impl Plugin for LevelTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((LevelTransitionMenuPlugin,));
    }
}

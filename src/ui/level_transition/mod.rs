use bevy::prelude::*;

pub use self::menu::FinishedLevelTransitionEvent;
use self::menu::LevelTransitionMenuPlugin;

mod menu;

pub struct LevelTransitionPlugin;

impl Plugin for LevelTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((LevelTransitionMenuPlugin,));
    }
}

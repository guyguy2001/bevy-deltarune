use bevy::math::Vec2;
use bevy::prelude::*;
// use ron::de::
use serde::Deserialize;

use crate::AppState;

fn default_as_false() -> bool {
    false
}

#[derive(Resource, Clone, Default, Deserialize)]
pub struct GameConfig {
    pub window_size: Vec2,
    #[serde(default)]
    pub fullscreen: bool,
    pub debug: Option<DebugConfig>,
    #[serde(default = "default_as_false")]
    pub infinite_hp: bool,
}

#[derive(Clone, Default, Deserialize)]
pub struct DebugConfig {
    pub starting_state: AppState,
}

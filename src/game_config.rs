use bevy::math::Vec2;
// use ron::de::
use serde::Deserialize;

use crate::AppState;

#[derive(Clone, Default, Deserialize)]
pub struct GameConfig {
    pub window_size: Vec2,
    #[serde(default)]
    pub fullscreen: bool,
    pub debug: Option<DebugConfig>,
}

#[derive(Clone, Default, Deserialize)]
pub struct DebugConfig {
    pub starting_state: AppState,
}

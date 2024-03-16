use bevy::math::Vec2;
// use ron::de::
use serde::Deserialize;

#[derive(Clone, Default, Deserialize)]
pub struct GameConfig {
    pub window_size: Vec2,
    #[serde(default)]
    pub fullscreen: bool,
}
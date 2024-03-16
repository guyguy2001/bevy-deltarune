use bevy::math::Vec2;
// use ron::de::
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct GameConfig {
    pub window_size: Vec2,
}
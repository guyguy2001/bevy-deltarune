use avian2d::prelude::PhysicsLayer;

#[derive(PhysicsLayer, Default)]
pub enum GameLayers {
    #[default]
    Default,
    Player,
    Bullet,
    Wall,
}

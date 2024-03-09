use bevy::{ecs::system::SystemId, prelude::*};
use rand::Rng;

pub struct Effect(pub SystemId<(), ()>);

#[derive(Default, Resource)]
pub struct LevelTransitionEffectsPool {
    effects: Vec<Effect>,
}

impl LevelTransitionEffectsPool {
    pub fn add_effect(&mut self, effect: Effect) {
        self.effects.push(effect);
    }

    pub fn get_random(&self) -> &Effect {
        &self.effects[rand::thread_rng().gen_range(0..self.effects.len())]
    }
}
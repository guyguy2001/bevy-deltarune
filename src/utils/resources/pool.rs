use bevy::prelude::*;
use rand::{seq::IteratorRandom, seq::SliceRandom};

/// A resource for managing a pool of selections we can pick from - for example the available upgrades, or the end-of-level effects.
#[derive(Resource)]
pub struct SelectionsPool<T: Clone> {
    items: Vec<T>,
}

impl<T: Clone> SelectionsPool<T> {
    pub fn add_item(&mut self, item: T) {
        self.items.push(item);
    }

    // pub fn get_random(&self) -> T {
    //     &self.items[rand::thread_rng().gen_range(0..self.items.len())]
    // }

    pub fn get_multiple_random(&self, count: usize) -> Vec<Option<T>> {
        let mut options: Vec<Option<T>> = self
            .items
            .iter()
            .choose_multiple(&mut rand::thread_rng(), count)
            .iter()
            .map(|item| Some((**item).clone()))
            .collect();
        options.append(&mut vec![None; count - options.len()]);
        options.shuffle(&mut rand::thread_rng());
        options
    }
}

impl<T: Clone> Default for SelectionsPool<T> {
    fn default() -> Self {
        // TODO: Why isn't this automatic when deriving?
        Self { items: Vec::new() }
    }
}

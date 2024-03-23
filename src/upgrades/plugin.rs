use bevy::prelude::*;
use std::path::Path;

use crate::utils::resources::pool::SelectionsPool;

pub struct UpgradesPlugin;

impl Plugin for UpgradesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectionsPool<Upgrade>>()
            .add_systems(Startup, populate_upgrades_pool);
    }
}

#[derive(Clone, Copy)]
pub struct Upgrade {
    pub name: &'static str,
    pub icon_texture: &'static Path,
}

fn populate_upgrades_pool(mut upgrade_pool: ResMut<SelectionsPool<Upgrade>>) {
    // TODO: Should this be a normal function called as an argument for app.insert_resource?
    upgrade_pool.add_item(Upgrade {
        name: "Sword",
        icon_texture: Path::new("sprites/upgrades/sword_diamond.png"),
    });
}

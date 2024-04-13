use std::path::Path;

use bevy::prelude::*;

use crate::{
    upgrades::{GlobalUpgrade, Upgrade, UpgradesReceiverFaction},
    utils::resources::pool::SelectionsPool,
};

use super::healthbar::Health;

pub fn populate_upgrades_pool(world: &mut World) {
    // mut upgrade_pool: ResMut<SelectionsPool<GlobalUpgrade>>
    // TODO: Should this be a normal function called as an argument for app.insert_resource?
    let unimplemented_id = world.register_system(unimplemented_system);
    let upgrades = vec![
        GlobalUpgrade {
            upgrade: Upgrade {
                apply_upgrade: world.register_system(add_10_health),
                name: "Apple",
                description: "Gain 10 max HP and heal them",
                icon_texture: Path::new("sprites/upgrades/apple.png"),
            },
            receiver_factions: UpgradesReceiverFaction::Player,
        },
        GlobalUpgrade {
            upgrade: Upgrade {
                apply_upgrade: unimplemented_id,
                name: "Sword",
                description: "Get a cool attack",
                icon_texture: Path::new("sprites/upgrades/sword_diamond.png"),
            },
            receiver_factions: UpgradesReceiverFaction::Player,
        },
        GlobalUpgrade {
            upgrade: Upgrade {
                apply_upgrade: world.register_system(make_bullets_yellow),
                name: "Yellow Boxes",
                description: "Test upgrade - make all enemy bullets yellow",
                icon_texture: Path::new("sprites/upgrade/yellow.png"),
            },
            receiver_factions: UpgradesReceiverFaction::EnemyBullets,
        },
    ];

    let mut upgrade_pool = world.resource_mut::<SelectionsPool<GlobalUpgrade>>();
    for upgrade in upgrades {
        upgrade_pool.add_item(upgrade);
    }
}

fn unimplemented_system(In(_entity): In<Entity>) {
    warn!("TODO: Unimplemented system was used");
}

/// TODO: How do I parametrize this so that I can create instances of this upgrade with arbitrary health amounts?
fn add_10_health(In(entity): In<Entity>, mut q_health: Query<&mut Health>) {
    if let Ok(mut health) = q_health.get_mut(entity) {
        health.health += 10.;
        health.max_health += 10.;
    }
}

fn make_bullets_yellow(In(entity): In<Entity>, mut sprite: Query<&mut Sprite>) {
    if let Ok(mut sprite) = sprite.get_mut(entity) {
        sprite.color = Color::YELLOW;
    }
}

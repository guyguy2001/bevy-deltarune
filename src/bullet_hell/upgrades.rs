use std::path::Path;

use bevy::{
    ecs::system::{BoxedSystem, FunctionSystem, IsFunctionSystem},
    prelude::*,
};

use crate::{
    upgrades::{GlobalUpgrade, Upgrade, UpgradesReceiverFaction},
    utils::resources::SelectionsPool,
};

use super::{health::Health, player::Player};

pub fn populate_upgrades_pool(world: &mut World) {
    // mut upgrade_pool: ResMut<SelectionsPool<GlobalUpgrade>>
    // TODO: Should this be a normal function called as an argument for app.insert_resource?
    let unimplemented_id = world.register_system(unimplemented_system);
    let upgrades = vec![
        GlobalUpgrade {
            upgrade: Upgrade {
                apply_upgrade: world
                    .register_boxed_system(AddHealthUpgrade::new(10.).into_boxed_system()),
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
                icon_texture: Path::new("sprites/upgrades/yellow.png"),
            },
            receiver_factions: UpgradesReceiverFaction::EnemyBullets,
        },
        GlobalUpgrade {
            upgrade: Upgrade {
                apply_upgrade: world.register_system(speed_player_up_by_10_percent),
                name: "Cool Shoes",
                description: "Speed player up by x1.1",
                icon_texture: Path::new("sprites/upgrades/minecart.png"),
            },
            receiver_factions: UpgradesReceiverFaction::Player,
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

// I don't like how complex this came out. Is it worth it? Is there a simpler way?
struct AddHealthUpgrade {
    amount: f32,
}

impl AddHealthUpgrade {
    pub fn new(amount: f32) -> Self {
        Self { amount }
    }

    fn apply(&self, In(entity): In<Entity>, mut q_health: Query<&mut Health>) {
        if let Ok(mut health) = q_health.get_mut(entity) {
            health.health += self.amount;
            health.max_health += self.amount;
        }
    }

    fn into_boxed_system(self) -> BoxedSystem<Entity, ()> {
        let system_function = move |In(entity): In<Entity>, q_health: Query<&mut Health>| {
            self.apply(In(entity), q_health);
        };

        Box::new(IntoSystem::into_system(system_function))
    }
}

// This is still here in case I want to remove the complicated AddHealthUpgrade version
fn _add_10_health(In(entity): In<Entity>, mut q_health: Query<&mut Health>) {
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

fn speed_player_up_by_10_percent(In(entity): In<Entity>, mut q_player: Query<&mut Player>) {
    if let Ok(mut player) = q_player.get_mut(entity) {
        player.speed *= 1.1;
    }
}

use std::{path::Path, time::Duration};

use bevy::{
    ecs::system::{BoxedSystem, SystemId},
    prelude::*,
};
use bevy_inspector_egui::prelude::*;

use crate::{
    // abilities::AbilityUpgradePool,
    upgrades::{GlobalUpgrade, Upgrade, UpgradesReceiverFaction},
    utils::resources::SelectionsPool,
};

use super::ability_lib::{self as lib, AbilityUpgradePool};

pub fn plugin(app: &mut App) {
    app.init_resource::<AbilityUpgradePool>() // TODO
        .add_systems(Startup, initialize_ability_upgrades_pool);
}

fn dash_system(_entity: In<Entity>) {
    println!("Dash");
}
fn parry_system(_entity: In<Entity>) {
    println!("parry");
}

fn initialize_ability_upgrades_pool(world: &mut World) {
    let dash_id = world.register_system(dash_system);
    let parry_id = world.register_system(parry_system);
    let abilities = [
        lib::Ability {
            activate: dash_id,
            cooldown: Duration::from_secs(1),
            description: "Dash",
            name: "Dash",
            icon_texture: Path::new("sprites/upgrades/minecart.png"),
        },
        lib::Ability {
            activate: parry_id,
            cooldown: Duration::from_secs_f32(0.5),
            description: "Cool sword",
            name: "Parry",
            icon_texture: Path::new("sprites/upgrades/sword_diamond.png"),
        },
    ];
    let upgrades: Vec<GlobalUpgrade> = abilities
        .iter()
        .map(|ability| GlobalUpgrade {
            upgrade: Upgrade {
                apply_upgrade: world
                    .register_boxed_system(SetAbility::new(ability.clone()).into_boxed_system()), // TODO: Remove clone
                name: ability.name,
                description: ability.description,
                icon_texture: ability.icon_texture,
            },
            receiver_factions: UpgradesReceiverFaction::Player,
        })
        .collect();

    let mut upgrade_pool = world.resource_mut::<AbilityUpgradePool>();
    for upgrade in upgrades {
        upgrade_pool.0.add_item(upgrade);
    }
}

// Todo: extract stuff to src/abilities (and probably modify that to src/lib/abilities or something)
struct SetAbility {
    ability: lib::Ability,
}

// TODO: I can probably make this generic over a system param, and have into_boxed_system be implemented by default
// TODO: This probably doesn't need to be a struct, it can just be a function
impl SetAbility {
    pub fn new(ability: lib::Ability) -> Self {
        Self { ability }
    }

    fn apply(&self, In(entity): In<Entity>, mut commands: Commands) {
        commands
            .entity(entity)
            .insert(lib::AbilityHolder::new(self.ability.clone()));
    }

    fn into_boxed_system(self) -> BoxedSystem<Entity, ()> {
        let system_function = move |In(entity): In<Entity>, commands: Commands| {
            self.apply(In(entity), commands);
        };

        Box::new(IntoSystem::into_system(system_function))
    }
}

use std::{path::Path, time::Duration};

use avian2d::prelude::CollisionLayers;
use bevy::{ecs::system::BoxedSystem, prelude::*};

use crate::{
    bullet_hell::{
        dash::{start_dashing, Dasher},
        player::ControllablePlayerFilter,
        sword::spawn_sword,
    },
    upgrades::{GlobalUpgrade, Upgrade, UpgradesReceiverFaction},
    utils::input::get_input_direction,
};

use super::ability_lib::{self as lib, AbilityUpgradePool};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, initialize_ability_upgrades_pool);
}

fn dash_system(
    entity: In<Entity>,
    mut commands: Commands,
    query: Query<(&Dasher, &CollisionLayers), ControllablePlayerFilter>,
    input: Res<ButtonInput<KeyCode>>,
) {
    println!("Dash");
    // TODO: Is it okay for this to reference the user input?
    let (dasher, collision_groups) = query.get(*entity).unwrap();
    start_dashing(
        *entity,
        get_input_direction(&input),
        dasher,
        collision_groups,
        &mut commands,
    );
}

fn initialize_ability_upgrades_pool(world: &mut World) {
    let dash_id = world.register_system(dash_system);
    let parry_id = world.register_system(spawn_sword);
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

    fn into_boxed_system(self) -> BoxedSystem<In<Entity>, ()> {
        let system_function = move |In(entity): In<Entity>, commands: Commands| {
            self.apply(In(entity), commands);
        };

        Box::new(IntoSystem::into_system(system_function))
    }
}

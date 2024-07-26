use std::path::Path;

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::upgrades::UpgradesReceiver;

use super::{GlobalUpgrade, Upgrade, UpgradesReceiverFaction};

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
struct HasCustomUpgrade;

fn add_custom_component(In(entity): In<Entity>, mut commands: Commands) {
    commands.entity(entity).insert(HasCustomUpgrade);
}

fn get_custom_upgrade(world: &mut World) -> GlobalUpgrade {
    GlobalUpgrade {
        receiver_factions: UpgradesReceiverFaction::EnemyBullets,
        upgrade: Upgrade {
            apply_upgrade: world.register_system(add_custom_component),
            description: "Test",
            name: "Test",
            icon_texture: Path::new(""),
        },
    }
}

#[test]
fn test_global_upgrades() {
    use crate::upgrades::{UpgradeApplier, UpgradesPlugin};

    let mut app = App::new();
    app.add_plugins(UpgradesPlugin);
    let mut world = app.world_mut();

    let should_receive_bundles = [
        (
            Name::new("Should receive 1"),
            UpgradesReceiver {
                factions: UpgradesReceiverFaction::EnemyBullets,
            },
        ),
        (
            Name::new("Should receive 2"),
            UpgradesReceiver {
                factions: UpgradesReceiverFaction::EnemyBullets | UpgradesReceiverFaction::Enemy,
            },
        ),
    ];
    let should_receive = should_receive_bundles
        .clone()
        .map(|bundle| world.spawn(bundle).id());

    let should_not_receive_bundles = [
        (
            Name::new("Should not receive 1"),
            UpgradesReceiver {
                factions: UpgradesReceiverFaction::Enemy,
            },
        ),
        (
            Name::new("Should not receive 2"),
            UpgradesReceiver {
                factions: UpgradesReceiverFaction::Enemy | UpgradesReceiverFaction::Player,
            },
        ),
    ];
    let should_not_receive = should_not_receive_bundles
        .clone()
        .map(|bundle| world.spawn(bundle).id());

    // No entity should have the upgrade before it is applied
    for entity in should_receive {
        assert!(world.entity(entity).get::<HasCustomUpgrade>().is_none());
    }
    for entity in should_not_receive {
        assert!(world.entity(entity).get::<HasCustomUpgrade>().is_none());
    }

    let upgrade = get_custom_upgrade(&mut world);
    let apply_upgrade = world.resource::<UpgradeApplier>().apply_upgrade_to_all;
    world.run_system_with_input(apply_upgrade, upgrade).unwrap();

    // Only the entities with the correct factions should receive the upgrade
    for entity in should_receive {
        assert!(world.entity(entity).get::<HasCustomUpgrade>().is_some());
    }
    for entity in should_not_receive {
        assert!(world.entity(entity).get::<HasCustomUpgrade>().is_none());
    }
    app.update();
    let mut world = app.world_mut();
    for entity in should_receive {
        assert!(world.entity(entity).get::<HasCustomUpgrade>().is_some());
    }
    for entity in should_not_receive {
        assert!(world.entity(entity).get::<HasCustomUpgrade>().is_none());
    }

    let should_receive_2 = should_receive_bundles.map(|bundle| world.spawn(bundle).id());
    let should_not_receive_2 = should_not_receive_bundles.map(|bundle| world.spawn(bundle).id());

    app.update();
    let mut world = app.world_mut();
    // Only the entities with the correct factions should receive the upgrade
    for entity in should_receive_2 {
        assert!(world.entity(entity).get::<HasCustomUpgrade>().is_some());
    }
    for entity in should_not_receive_2 {
        assert!(world.entity(entity).get::<HasCustomUpgrade>().is_none());
    }
}

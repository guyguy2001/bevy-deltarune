use bevy::{ecs::system::SystemId, prelude::*};
use bevy_inspector_egui::prelude::*;
use bitmask_enum::bitmask;
use std::path::Path;

use crate::utils::resources::pool::SelectionsPool;

pub struct UpgradesPlugin;

impl Plugin for UpgradesPlugin {
    fn build(&self, app: &mut App) {
        let apply_system_id = app.world.register_system(apply_upgrade_to_all);
        app.init_resource::<SelectionsPool<GlobalUpgrade>>()
            .init_resource::<AppliedGlobalUpgrades>();
        app.insert_resource(UpgradeApplier {
            apply_upgrade_to_all: apply_system_id,
        })
        .add_systems(PostUpdate, apply_upgrade_on_spawn); // TODO: Correct schedule to not flicker
    }
}

#[derive(Clone, Copy)]
pub struct Upgrade {
    pub apply_upgrade: SystemId<Entity, ()>,
    pub name: &'static str,
    pub description: &'static str,
    pub icon_texture: &'static Path,
}

#[bitmask]
#[derive(InspectorOptions, Default, Reflect)]
#[reflect(InspectorOptions)]
pub enum UpgradesReceiverFaction {
    Player,
    Enemy,
    EnemyBullets,
}

/// A marker for an entity that can receive upgrades.
#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct UpgradesReceiver {
    pub factions: UpgradesReceiverFaction,
}

fn should_apply_upgrade(receiver: &UpgradesReceiver, upgrade: &GlobalUpgrade) -> bool {
    !upgrade.receiver_factions.or(receiver.factions).is_none()
}

fn apply_upgrade_on_spawn(
    q_receivers: Query<(Entity, &UpgradesReceiver), Added<UpgradesReceiver>>, // Todo: Added or spawn?
    mut commands: Commands,
    applied_global_upgrades: Res<AppliedGlobalUpgrades>,
) {
    for (entity, receiver) in q_receivers.iter() {
        for upgrade in applied_global_upgrades.applied_upgrades.iter() {
            if should_apply_upgrade(receiver, upgrade) {
                commands.run_system_with_input(upgrade.upgrade.apply_upgrade, entity);
            }
        }
    }
}

// TODO:
// 1. V replace the construction of Upgrade with GlobalUpgrade
// 2. V call apply_upgrade_to_all instead of calling the upgrade directly in the menu
// 3. Append the upgrade to a relevant resource.

#[derive(Clone)]
pub struct GlobalUpgrade {
    pub upgrade: Upgrade,
    pub receiver_factions: UpgradesReceiverFaction,
}

#[derive(Resource)]
pub struct UpgradeApplier {
    pub apply_upgrade_to_all: SystemId<GlobalUpgrade>,
}

// TODO: Name
fn apply_upgrade_to_all(
    In(upgrade): In<GlobalUpgrade>,
    mut commands: Commands,
    q_relevant_entities: Query<(Entity, &UpgradesReceiver)>,
    mut applied_global_upgrades: ResMut<AppliedGlobalUpgrades>,
) {
    for (entity, receiver) in q_relevant_entities.iter() {
        if should_apply_upgrade(&receiver, &upgrade) {
            commands.run_system_with_input(upgrade.upgrade.apply_upgrade, entity)
        }
    }
    applied_global_upgrades.applied_upgrades.push(upgrade);
}

#[derive(Resource, Default)]
pub struct AppliedGlobalUpgrades {
    pub applied_upgrades: Vec<GlobalUpgrade>,
}

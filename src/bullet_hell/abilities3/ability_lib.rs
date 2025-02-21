use std::{path::Path, time::Duration};

use bevy::{ecs::system::BoxedSystem, ecs::system::SystemId, prelude::*};
use bevy_inspector_egui::prelude::*;

use crate::{upgrades::GlobalUpgrade, utils::resources::SelectionsPool};

#[derive(Resource, Default)]
pub struct AbilityUpgradePool(pub SelectionsPool<GlobalUpgrade>);

#[derive(Clone, Reflect)]
#[reflect(from_reflect = false)]
pub struct Ability {
    pub cooldown: Duration, // TODO: But don't I want the duration to be configurable per instance of the same ability type?
    #[reflect(ignore)]
    pub activate: SystemId<Entity, ()>,
    pub name: &'static str,
    pub description: &'static str,
    pub icon_texture: &'static Path,
    // I'm not adding a hotkey per ability yet, the player will have just 1 ability
    // pub hotkey: KeyCode // TODO: What if I have 2 abilities of the same type (on separate cooldowns)? Incredibly freaking out of scope, but still
}

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
#[reflect(from_reflect = false)]
pub struct AbilityHolder {
    ability: Ability,
    cooldown: Timer,
}

impl AbilityHolder {
    pub fn new(ability: Ability) -> Self {
        Self {
            cooldown: Timer::new(ability.cooldown, TimerMode::Once),
            ability,
        }
    }
}

fn activate_ability_on_space(
    mut q_holder: Query<&mut AbilityHolder>,
    input: Res<ButtonInput<KeyCode>>,
) {
    // if input.just_pressed(KeyCode::Space) {
    //     for holder_mut in q_holder.iter_mut() {
    //         holder_mut.field_mut
    //         let holder_pure: AbilityHolder = todo!();
    //         holder_pure.
    //         // if holder.cooldown.finished() {
    //         //     holder.cooldown.reset();
    //         //     // holder.ability.
    //         // }
    //     }
    // }
}

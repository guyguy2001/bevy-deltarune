use std::{path::Path, time::Duration};

use bevy::{ecs::system::SystemId, prelude::*};
use bevy_inspector_egui::prelude::*;

use crate::{upgrades::GlobalUpgrade, utils::resources::SelectionsPool};

pub fn plugin(app: &mut App) {
    app.init_resource::<AbilityUpgradePool>()
        .register_type::<AbilityHolder>()
        .add_systems(Update, abilities_activation);
}

#[derive(Resource, Default)]
pub struct AbilityUpgradePool(pub SelectionsPool<GlobalUpgrade>);

#[derive(Clone, Reflect)]
#[reflect(from_reflect = false)]
pub struct Ability {
    pub cooldown: Duration, // TODO: But don't I want the duration to be configurable per instance of the same ability type?
    #[reflect(ignore)]
    pub activate: SystemId<In<Entity>, ()>,
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

    pub fn ability(&self) -> &Ability {
        &self.ability
    }

    pub fn cooldown(&self) -> &Timer {
        &self.cooldown
    }
}

#[derive(Event)]
pub struct OnAbilityChange;

fn abilities_activation(
    mut commands: Commands,
    mut q_holder: Query<(Entity, &mut AbilityHolder)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (entity, mut holder) in q_holder.iter_mut() {
        holder.cooldown.tick(time.delta());
        if input.just_pressed(KeyCode::Space) {
            println!("Ability");
            if holder.cooldown.finished() {
                holder.cooldown.reset();
                commands.run_system_with(holder.ability.activate, entity);
            }
        }
    }
}

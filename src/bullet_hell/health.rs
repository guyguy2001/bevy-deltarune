use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::AppState;

pub struct HealthPlugin;
impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnDamageDealt>()
            .register_type::<Health>()
            .add_systems(Update, handle_damage.run_if(in_state(AppState::Defending)));
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Health {
    pub health: f32,
    // TODO: How I make it public only to stuff like the upgrade system?
    pub max_health: f32,
}

impl Health {
    pub fn new(max_health: f32) -> Self {
        Health {
            health: max_health,
            max_health,
        }
    }
}

#[derive(Event)]
pub struct OnDamageDealt {
    pub target_entity: Entity,
    pub damage: f32,
}

fn handle_damage(mut reader: EventReader<OnDamageDealt>, mut query: Query<&mut Health>) {
    for event in reader.read() {
        let mut health = query.get_mut(event.target_entity).unwrap();
        health.health -= event.damage;
    }
}

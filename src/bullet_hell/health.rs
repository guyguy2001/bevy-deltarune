use std::time::Duration;

use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::AppState;

pub struct HealthPlugin;
impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TryDamageEvent>()
            .add_event::<DamageOccurredEvent>()
            .register_type::<Health>()
            .add_systems(
                Update,
                (handle_damage, handle_invulnerability).run_if(in_state(AppState::Defending)),
            );
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
pub struct TryDamageEvent {
    pub target_entity: Entity,
    pub damage: f32,
}

#[derive(Event)]
pub struct DamageOccurredEvent {
    pub target_entity: Entity,
    #[allow(dead_code)]
    pub damage: f32,
}

fn handle_damage(
    mut reader: EventReader<TryDamageEvent>,
    mut damage_occurred_writer: EventWriter<DamageOccurredEvent>,
    mut query: Query<(Entity, &mut Health, Has<Invulnerability>)>,
    mut commands: Commands,
) {
    for event in reader.read() {
        let (entity, mut health, has_invul) = query.get_mut(event.target_entity).unwrap();
        if !has_invul {
            damage_occurred_writer.send(DamageOccurredEvent {
                target_entity: event.target_entity,
                damage: event.damage,
            });
            health.health -= event.damage;
            commands
                .entity(entity)
                .insert(Invulnerability::new(Duration::from_secs(1)));
        }
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Invulnerability {
    timer: Timer,
}

impl Invulnerability {
    pub fn new(duration: Duration) -> Self {
        Self {
            timer: Timer::new(duration, TimerMode::Once),
        }
    }
}

fn handle_invulnerability(
    mut query: Query<(Entity, &mut Invulnerability)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut invulnerability) in query.iter_mut() {
        invulnerability.timer.tick(time.delta());
        if invulnerability.timer.just_finished() {
            commands.entity(entity).remove::<Invulnerability>();
        }
    }
}

use std::time::Duration;

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_rapier2d::{
    control::KinematicCharacterController, dynamics::Velocity, geometry::CollisionGroups,
};

use crate::{utils::input::get_input_direction, AppState};

use super::{
    health::Invulnerability,
    physics_layers,
    player::{ControllablePlayerFilter, ControlledExternally},
};

pub struct DashPlugin;

impl Plugin for DashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (start_dashing_on_input, handle_active_dash).run_if(in_state(AppState::Defending)),
        );
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Dasher {
    pub dash_amount: f32,
    pub dash_speed: f32,
}

// TODO: Add the hit_effects stuff to a system_set/schedule that will happen after the gameplay ones
// TODO: Should I reference the player? Should I even handle input directly here?
// TODO: cooldown

fn start_dashing_on_input(
    query: Query<(Entity, &Dasher, &CollisionGroups), ControllablePlayerFilter>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::Space) {
        for (entity, dasher, collision_groups) in query.iter() {
            start_dashing(
                entity,
                get_input_direction(&input),
                dasher,
                collision_groups,
                &mut commands,
            );
        }
    }
}

fn start_dashing(
    entity: Entity,
    direction: Vec3,
    dasher: &Dasher,
    original_collision_groups: &CollisionGroups,
    commands: &mut Commands,
) {
    if direction != Vec3::ZERO {
        let dash_duration = Duration::from_secs_f32(dasher.dash_amount / dasher.dash_speed);
        commands.entity(entity).insert((
            ActiveDash::new(
                dash_duration,
                (direction * dasher.dash_speed).xy(),
                *original_collision_groups,
            ),
            ControlledExternally::new("Dash"),
            Invulnerability::new(dash_duration),
            CollisionGroups::new(
                original_collision_groups.memberships,
                original_collision_groups.filters & !physics_layers::BULLETS,
            ),
        ));
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
struct ActiveDash {
    timer: Timer,
    linear_velocity: Vec2,
    original_collision_groups: CollisionGroups,
}

impl ActiveDash {
    // TODO: Maybe create a OnceTimer, and make it public, since I duplicate this constructor everywhere?
    pub fn new(
        dash_duration: Duration,
        linear_velocity: Vec2,
        original_collision_groups: CollisionGroups,
    ) -> Self {
        Self {
            timer: Timer::new(dash_duration, TimerMode::Once),
            linear_velocity,
            original_collision_groups,
        }
    }
}

fn handle_active_dash(
    mut query: Query<(Entity, &mut ActiveDash, &mut KinematicCharacterController)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut dash, mut controller) in query.iter_mut() {
        // TODO: Extract movement to FixedUpdate
        controller.translation = Some(dash.linear_velocity * time.delta().as_secs_f32());

        dash.timer.tick(time.delta());
        if dash.timer.just_finished() {
            commands
                .entity(entity)
                .remove::<ControlledExternally>()
                .remove::<ActiveDash>()
                .remove::<Velocity>()
                .insert(dash.original_collision_groups);
        }
    }
}

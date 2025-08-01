use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::{
    upgrades::{UpgradesReceiver, UpgradesReceiverFaction},
    AppState,
};

use super::{game_z_index, health::TryDamageEvent, physics_layers, player::Player};

pub struct BulletsPlugin;

impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            player_collision.run_if(in_state(AppState::Defending)),
        );
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
// TODO: Remove the `pub` by making whatever this is needed for a more general thing
pub struct Bullet {
    pub direction: Vec3,
    pub damage: f32,
    pub speed: f32,
}

pub struct BulletProperties {
    pub damage: f32,
    pub size: f32,
    pub speed: f32,
}

pub fn spawn_bullet_in_pos(
    position: Vec3,
    direction: Vec3,
    properties: BulletProperties,
    commands: &mut Commands,
) {
    // TODO: Question - when do I receive asset_server as a parameter, and when do I get it from the world?
    // TODO: ask for the asset_server inside of the commands queue, instead of directly here?
    commands.queue(move |world: &mut World| {
        let asset_server = world.get_resource::<AssetServer>().unwrap(); // TODO: How do I not unwrap?
        world.spawn((
            Sprite {
                custom_size: Some(Vec2::new(properties.size, properties.size)),
                ..Sprite::from_image(asset_server.load("pig.png"))
            },
            Transform {
                translation: Vec3::new(position.x, position.y, game_z_index::BULLETS),
                ..Default::default()
            },
            Bullet {
                direction,
                damage: properties.damage,
                speed: properties.speed,
            },
            UpgradesReceiver {
                factions: UpgradesReceiverFaction::EnemyBullets,
            },
            (
                CollisionLayers::new(
                    physics_layers::GameLayers::Bullet,
                    physics_layers::GameLayers::all_bits(),
                ),
                RigidBody::Kinematic,
                Collider::rectangle(properties.size, properties.size),
                Sensor,
                LinearVelocity((direction * properties.speed).xy()),
            ),
        ));
    });
}

// fn bullet_movement(
// TODO: Make sure they don't move during the cutscene (maybe avian supports this built in?)
//     mut query: Query<(&mut KinematicCharacterController, &Bullet)>,
//     time: Res<Time>,
// ) {
//     for (mut controller, bullet) in &mut query {
//         let movement_amount = bullet.speed * time.delta_seconds();
//         controller.translation = Some(bullet.direction.normalize().xy() * movement_amount);
//     }
// }

fn player_collision(
    mut commands: Commands,
    mut contact_events: EventReader<CollisionStarted>,
    bullets: Query<(Entity, &Bullet)>,
    mut players: Query<Entity, With<Player>>,
    mut damage_events: EventWriter<TryDamageEvent>,
) {
    for event in contact_events.read() {
        let CollisionStarted(entity1, entity2) = event;
        // TODO: get this working with swapped entity orders???
        println!("Collision: {entity1} {entity2}");
        let (player_entity, (bullet_entity, bullet_component)) = if let (Ok(player), Ok(enemy)) =
            (players.get_mut(*entity1), bullets.get(*entity2))
        {
            (player, enemy)
        } else if let (Ok(player), Ok(enemy)) = (players.get_mut(*entity2), bullets.get(*entity1)) {
            (player, enemy)
        } else {
            return;
        };
        commands.entity(bullet_entity).despawn();

        // TODO: make this an event? Who is responsible for handling it? what would it achieve?
        damage_events.write(TryDamageEvent {
            target_entity: player_entity,
            damage: bullet_component.damage,
        });
    }
}

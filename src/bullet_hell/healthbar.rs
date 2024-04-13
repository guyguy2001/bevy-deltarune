use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::utils::world_ui::WorldUI;
use crate::utils::z_index;
use crate::AppState;

pub struct HealthbarPlugin;
impl Plugin for HealthbarPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Health>().add_systems(
            Update,
            healthbar_behaviour.run_if(in_state(AppState::Defending)),
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
            max_health: max_health,
        }
    }
}

#[derive(Component)]
struct Healthbar {
    tracked_entity: Entity,
}

#[derive(Component)]
struct GreenPart;

pub fn spawn_healthbar(commands: &mut Commands, character_entity: Entity) {
    commands
        .spawn((
            NodeBundle {
                z_index: z_index::WORLD_UI,
                style: Style {
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            WorldUI {
                tracked_entity: character_entity,
            },
            Name::new("Healthbar WorldUI"),
        ))
        .with_children(|commands| {
            commands
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(100.),
                            height: Val::Px(30.),
                            bottom: Val::Px(50.),
                            border: UiRect::all(Val::Px(5.)),
                            justify_content: JustifyContent::Start,
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        border_color: Color::BLACK.into(),
                        ..default()
                    },
                    Name::new("Healthbar"),
                ))
                .with_children(|commands| {
                    commands.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                ..default()
                            },
                            background_color: Color::GREEN.into(),
                            ..default()
                        },
                        GreenPart,
                        Healthbar {
                            tracked_entity: character_entity,
                        },
                    ));
                });
        });
}

fn healthbar_behaviour(
    health_query: Query<&Health>,
    mut healthbar_query: Query<(&mut Style, &Healthbar), With<GreenPart>>,
) {
    for (mut style, healthbar) in healthbar_query.iter_mut() {
        let health_component = health_query.get(healthbar.tracked_entity).unwrap();
        style.width = Val::Percent(100. * health_component.health / health_component.max_health);
    }
}

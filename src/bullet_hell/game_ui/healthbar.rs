use bevy::prelude::*;

use crate::{
    bullet_hell::health::Health,
    ui,
    utils::{world_ui::WorldUI, z_index},
    AppState,
};

pub struct HealthbarPlugin;

impl Plugin for HealthbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            healthbar_behaviour.run_if(in_state(AppState::Defending)),
        );
    }
}

#[derive(Component)]
struct Healthbar {
    tracked_entity: Entity,
}

#[derive(Component)]
struct GreenPart;

pub fn spawn_healthbar(commands: &mut Commands, character_entity: Entity) {
    commands.spawn((
        z_index::WORLD_UI,
        Node {
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        WorldUI {
            tracked_entity: character_entity,
        },
        Name::new("Healthbar WorldUI"),
        children![(
            Node {
                width: Val::Px(100.),
                height: Val::Px(30.),
                bottom: Val::Px(50.),
                border: UiRect::all(Val::Px(5.)),
                justify_content: JustifyContent::Start,
                position_type: PositionType::Absolute,
                ..default()
            },
            BorderColor(ui::palette::BLACK.into()),
            Name::new("Healthbar"),
            children![(
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                BackgroundColor(ui::palette::GREEN.into()),
                GreenPart,
                Healthbar {
                    tracked_entity: character_entity,
                },
            )],
        )],
    ));
}

fn healthbar_behaviour(
    health_query: Query<&Health>,
    mut healthbar_query: Query<(&mut Node, &Healthbar), With<GreenPart>>,
) {
    for (mut node, healthbar) in healthbar_query.iter_mut() {
        let health_component = health_query.get(healthbar.tracked_entity).unwrap();
        node.width = Val::Percent(100. * health_component.health / health_component.max_health);
    }
}

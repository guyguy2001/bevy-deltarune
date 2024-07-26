use std::cmp::Ordering;

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::ui;

use super::plugin::{GameStep, MetagameProgression};

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct LevelText(usize);

fn steps_to_strings<'a, T: Iterator<Item = &'a GameStep>>(game_steps: T) -> Vec<String> {
    let mut level_index = 0;

    game_steps
        .map(|step| match step {
            GameStep::Level(_) => {
                level_index += 1;
                format!("Level {level_index}")
            }
            GameStep::UpgradeShop => "Shop".into(),
        })
        .collect()
}

pub fn spawn_menu(mut commands: Commands, progress: Res<MetagameProgression>) {
    commands
        // .ui_builder(UiRoot)
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                top: Val::Percent(50.),
                bottom: Val::Percent(50.),
                left: Val::Px(25.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            for (i, name) in steps_to_strings(progress.iter_levels()).iter().enumerate() {
                builder
                    .spawn((NodeBundle {
                        style: Style {
                            ..Default::default()
                        },
                        ..Default::default()
                    },))
                    .with_children(|builder| {
                        builder.spawn((
                            TextBundle {
                                text: Text::from_section(
                                    name,
                                    TextStyle {
                                        font_size: 32.0,
                                        ..default()
                                    },
                                ),
                                ..Default::default()
                            },
                            LevelText(i),
                        ));
                    });
            }
        });
}

pub fn update_text_on_level_transition(
    mut query: Query<(&mut Text, &LevelText)>,
    progress: Res<MetagameProgression>,
) {
    if progress.is_changed() {
        for (mut style, level) in query.iter_mut() {
            match level.0.cmp(&progress.current_step_index) {
                Ordering::Less => style.sections[0].style.color = ui::palette::BLACK.into(),
                Ordering::Equal => style.sections[0].style.color = ui::palette::GREEN.into(),
                Ordering::Greater => style.sections[0].style.color = ui::palette::WHITE.into(),
            }
        }
    }
}

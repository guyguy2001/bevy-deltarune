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
            GameStep::AbilityShop => "Ability Shop".into(),
            GameStep::UpgradeShop => "Shop".into(),
        })
        .collect()
}

pub fn spawn_menu(mut commands: Commands, progress: Res<MetagameProgression>) {
    commands
        // .ui_builder(UiRoot)
        .spawn(Node {
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            top: Val::Percent(50.),
            bottom: Val::Percent(50.),
            left: Val::Px(25.),
            ..Default::default()
        })
        .with_children(|builder| {
            for (i, name) in steps_to_strings(progress.iter_levels()).iter().enumerate() {
                builder.spawn(Node::default()).with_children(|builder| {
                    builder.spawn((
                        Text(name.into()),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        LevelText(i),
                    ));
                });
            }
        });
}

pub fn update_text_on_level_transition(
    mut query: Query<(&mut TextColor, &LevelText)>,
    progress: Res<MetagameProgression>,
) {
    if progress.is_changed() {
        for (mut color, level) in query.iter_mut() {
            match level.0.cmp(&progress.current_step_index) {
                Ordering::Less => *color = ui::palette::BLACK.into(),
                Ordering::Equal => *color = ui::palette::GREEN.into(),
                Ordering::Greater => *color = ui::palette::WHITE.into(),
            }
        }
    }
}

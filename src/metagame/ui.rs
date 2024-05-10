// use crate::utils::sickle::ui_builder::{UiBuilderExt, UiContextRoot, UiRoot};
// use crate::utils::sickle::ui_commands::SetCursorExt;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use super::plugin::MetagameProgression;

// use crate::utils::sickle::ui_style::{SetBackgroundColorExt, SetNodeHeightExt, SetNodeWidthExt};
#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct LevelText;

pub fn spawn_menu(mut commands: Commands) {
    commands
        // .ui_builder(UiRoot)
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(50.),
                left: Val::Px(25.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Level 1",
                        TextStyle {
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    ..Default::default()
                },
                LevelText,
            ));
        });

    //    commands .ui_builder(UiRoot).
}

pub fn update_text_on_level_transition(
    mut query: Query<&mut Text, With<LevelText>>,
    progress: Res<MetagameProgression>,
) {
    if progress.is_changed() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("Level {}", progress.current_level);
        }
    }
}

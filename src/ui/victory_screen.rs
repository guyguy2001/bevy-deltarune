use bevy::prelude::*;

use crate::{ui, utils::z_index, AppState};

pub struct VictoryScreenPlugin;

impl Plugin for VictoryScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Victory), spawn_victory_popup);
    }
}

fn spawn_victory_popup(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                z_index: z_index::POPUP_MENU,
                style: Style {
                    width: Val::Percent(40.0),
                    height: Val::Percent(40.0),
                    left: Val::Percent(30.),
                    bottom: Val::Percent(30.),
                    border: UiRect::all(Val::Px(5.)),
                    justify_content: JustifyContent::SpaceAround,
                    position_type: PositionType::Absolute,
                    display: Display::Flex,
                    ..default()
                },
                background_color: ui::palette::DARK_GRAY.into(),
                border_color: Color::BLACK.into(),
                ..default()
            },
            Name::new("Victory Screen"),
        ))
        .with_children(|commands| {
            commands.spawn((TextBundle {
                text: Text::from_section(
                    "You Win!",
                    TextStyle {
                        font_size: 32.0,
                        ..default()
                    },
                ),
                ..default()
            },));
        });
}

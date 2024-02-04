use bevy::prelude::*;

use crate::AppState;

pub struct LevelTransitionMenuPlugin;

impl Plugin for LevelTransitionMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::LevelTransition),
            spawn_level_transition_menu,
        );
    }
}

fn spawn_level_transition_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
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
                background_color: Color::DARK_GRAY.into(),
                border_color: Color::BLACK.into(),
                ..default()
            },
            Name::new("Level Transition"),
        ))
        .with_children(|commands| {
            commands.spawn((TextBundle {
                text: Text::from_section(
                    "You win!",
                    TextStyle {
                        font_size: 32.0,
                        ..default()
                    },
                ),
                ..default()
            },));
        });
}

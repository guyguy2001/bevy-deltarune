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
            Node {
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
            BackgroundColor(ui::palette::DARK_GRAY.into()),
            BorderColor(Color::BLACK.into()),
            z_index::POPUP_MENU,
            Name::new("Victory Screen"),
        ))
        .with_children(|commands| {
            commands.spawn((
                Text("You Win!".into()),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
            ));
        });
}

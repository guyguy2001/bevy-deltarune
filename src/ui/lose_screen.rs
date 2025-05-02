use bevy::prelude::*;

use crate::{utils::z_index, AppState};

use super::palette;

#[derive(Event)]
pub struct LoseEvent;

pub struct LoseScreenPlugin;

impl Plugin for LoseScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoseEvent>()
            .add_systems(Update, transition_to_defeat)
            .add_systems(OnEnter(AppState::Defeat), spawn_menu_ui);
    }
}

fn transition_to_defeat(
    mut lose_event: EventReader<LoseEvent>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for _ in lose_event.read() {
        app_state.set(AppState::Defeat)
    }
}

fn spawn_menu_ui(mut commands: Commands) {
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
            BackgroundColor(palette::DARK_GRAY.into()),
            BorderColor(Color::BLACK.into()),
            z_index::POPUP_MENU,
            Name::new("Lose Screen"),
        ))
        .with_children(|commands| {
            commands.spawn((
                Text("You Lose!".into()),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
            ));
        });
}

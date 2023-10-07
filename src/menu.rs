use bevy::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct MenuUI;

impl Plugin for MenuUI {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_menu_ui);
            // .add_systems(Update, update_money_ui);
    }
}

#[derive(Component)]
struct MoneyText;

#[derive(Component, strum_macros::EnumIter, strum_macros::Display)]
enum Buttons {
    ATTACK,
    ACT,
    ITEMS,
    RUN,
}

fn spawn_menu_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(30.0),
                    // align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.),
                    bottom: Val::Px(0.),
                    border: UiRect::all(Val::Px(20.)),
                    display: Display::Flex,
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..default()
            },
            Name::new("UI Root"),
        ))
        .with_children(|commands: &mut ChildBuilder<'_, '_, '_>| {
            for button in Buttons::iter() {
                commands.spawn((
                    TextBundle {
                        text: Text::from_section(
                            button.to_string(),
                            TextStyle {
                                font_size: 32.0,
                                ..default()
                            },
                        ),
                        ..default()
                    },
                    // MoneyText,
                ));
            }
        });
}

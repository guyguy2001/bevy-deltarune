use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::utils::data_structures::Index;

pub struct MenuUI;

impl Plugin for MenuUI {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_menu_ui)
            .add_systems(Update, select_action);
    }
}

#[derive(Component)]
pub struct MultiChoiceParent {
    pub selected: Index,
}

#[derive(Component)]
pub struct MultiChoiceButton;

#[derive(Component, PartialEq, Eq, strum_macros::EnumIter, strum_macros::Display)]
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
                    left: Val::Px(0.),
                    bottom: Val::Px(0.),
                    border: UiRect::all(Val::Px(20.)),
                    justify_content: JustifyContent::SpaceAround,
                    position_type: PositionType::Absolute,
                    display: Display::Flex,
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..default()
            },
            MultiChoiceParent {
                selected: Index::new(4, 0),
            },
            Name::new("UI Root"),
        ))
        .with_children(|commands| {
            for button in Buttons::iter() {
                commands
                    .spawn((
                        NodeBundle {
                            style: Style {
                                border: UiRect::all(Val::Px(5.)),
                                width: Val::Percent(20.),
                                height: Val::Auto,
                                align_items: AlignItems::Start,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            border_color: if button == Buttons::ATTACK {
                                Color::GREEN.into()
                            } else {
                                Color::BLACK.into()
                            },
                            ..default()
                        },
                        MultiChoiceButton,
                    ))
                    .with_children(|commands| {
                        commands.spawn((TextBundle {
                            text: Text::from_section(
                                button.to_string(),
                                TextStyle {
                                    font_size: 32.0,
                                    ..default()
                                },
                            ),
                            ..default()
                        },));
                    });
            }
        });
}

fn deactivate(button: &mut BorderColor) {
    (*button).0 = Color::BLACK;
}

fn activate(button: &mut BorderColor) {
    (*button).0 = Color::GREEN;
}

fn change_selection(
    multi_choice_parent: &mut Mut<MultiChoiceParent>,
    buttons: &mut Query<&mut BorderColor, With<MultiChoiceButton>>,
    children: &Children,
    change_amount: i8,
) {
    deactivate(
        &mut buttons
            .get_mut(*children.get(multi_choice_parent.selected.index).unwrap())
            .unwrap(),
    );

    multi_choice_parent.selected.add(change_amount);
    println!("selected: {:?}", multi_choice_parent.selected);

    activate(
        &mut buttons
            .get_mut(*children.get(multi_choice_parent.selected.index).unwrap())
            .unwrap(),
    );
}

fn select_action(
    mut parent: Query<(Entity, &Children, &mut MultiChoiceParent)>,
    mut buttons_query: Query<&mut BorderColor, With<MultiChoiceButton>>,
    input: Res<Input<KeyCode>>,
) {
    let (_, children, mut multi_choice_parent) = parent.get_single_mut().unwrap();
    if input.just_pressed(KeyCode::Right) {
        change_selection(&mut multi_choice_parent, &mut buttons_query, &children, 1);
    } else if input.just_pressed(KeyCode::Left) {
        change_selection(&mut multi_choice_parent, &mut buttons_query, &children, -1);
    }
}

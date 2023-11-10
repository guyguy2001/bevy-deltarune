use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::utils::data_structures::Index;

pub struct MenuUI;

impl Plugin for MenuUI {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_menu_ui)
            .add_systems(Update, (select_action, activate_action));
    }
}

type MenuCallback = fn(String);

#[derive(Component)]
pub struct MultiChoiceParent {
    pub selected: Index,
}

#[derive(Component)]
pub struct MultiChoiceButton {
    text: String,
    callback: MenuCallback,
}

#[derive(Component, PartialEq, Eq, strum_macros::EnumIter, strum_macros::Display)]
enum Buttons {
    ATTACK,
    ACT,
    ITEMS,
    RUN,
}

const MENU_ITEMS: [(Buttons, MenuCallback); 4] = [
    (Buttons::ATTACK, spawn_attack_menu),
    (Buttons::ACT, not_supported),
    (Buttons::ITEMS, not_supported),
    (Buttons::RUN, not_supported),
];

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
                selected: Index::new(MENU_ITEMS.len(), 0),
            },
            Name::new("UI Root"),
        ))
        .with_children(|commands| {
            for (name, callback) in MENU_ITEMS.iter() {
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
                            border_color: if *name == Buttons::ATTACK {
                                Color::GREEN.into()
                            } else {
                                Color::BLACK.into()
                            },
                            ..default()
                        },
                        MultiChoiceButton {
                            text: name.to_string(),
                            callback: *callback,
                        },
                    ))
                    .with_children(|commands| {
                        commands.spawn((TextBundle {
                            text: Text::from_section(
                                name.to_string(),
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

fn get_selected_child(multi_choice_parent: &Mut<MultiChoiceParent>, children: &Children) -> Entity {
    // The unwrap is safe if and only if multi_choice_parent was defined to handle the given children.
    return children
        .get(multi_choice_parent.selected.index)
        .unwrap()
        .clone(); // TODO: Make sure cloning is safe; I'm pretty sure it is, as it also derives Copy
}

fn change_selection(
    multi_choice_parent: &mut Mut<MultiChoiceParent>,
    buttons: &mut Query<&mut BorderColor, With<MultiChoiceButton>>,
    children: &Children,
    change_amount: i8,
) {
    deactivate(
        &mut buttons
            .get_mut(get_selected_child(multi_choice_parent, children))
            .unwrap(),
    );

    multi_choice_parent.selected.add(change_amount);

    activate(
        &mut buttons
            .get_mut(get_selected_child(multi_choice_parent, children))
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

fn activate_action(
    mut parent: Query<(&Children, &mut MultiChoiceParent)>,
    buttons_query: Query<&MultiChoiceButton>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) || input.just_pressed(KeyCode::Return) {
        let (children, multi_choice) = parent.single_mut();
        // TODO: This would have been simpler if I could just say multi_choice_parent.get_selected_child()
        // Maybe a helper function would do that? maybe a custom query?
        let selected = buttons_query
            .get(get_selected_child(&multi_choice, children))
            .unwrap();
        (selected.callback)(selected.text.clone());
    }
}

fn not_supported(button_text: String) {
    warn!("{} is not supported yet!", button_text);
}

fn spawn_attack_menu(text: String) {
    // How do I change the game state from here? Do I need a reference to the world?
    // Check in the tutorial how he changes the states (probably from a query)
    // Does it make sense that the system knows what the callbacks might influence?
    info!("todo - spawn {} menu", text);
}

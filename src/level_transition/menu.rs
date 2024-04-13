use bevy::prelude::*;

use crate::{
    upgrades::{GlobalUpgrade, UpgradeApplier},
    utils::{
        data_structures::Index,
        menu_system::{MenuStack, MultiChoiceButton, MultiChoiceParent, SpawnedMenu},
        resources::pool::SelectionsPool,
        z_index,
    },
    AppState,
};

pub struct LevelTransitionMenuPlugin;

impl Plugin for LevelTransitionMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::LevelTransition),
            (spawn_level_transition_menu, register_menu).chain(),
        )
        .add_systems(
            OnExit(AppState::LevelTransition),
            (unregister_menu, despawn_level_transition_menu).chain(),
        );
    }
}

/// The root of the upgrade select menu - the entire thing that gets spawned/despawned when changing states
#[derive(Component)]
struct UpgradeSelectMenuRoot;

/// The direct parent of the upgrade menu buttons, for registering the menu
#[derive(Component)]
struct UpgradeSelectMenu;

#[derive(Component)]
struct UpgradeOption(Option<GlobalUpgrade>);

const MENU_ITEMS: [i32; 3] = [1, 2, 3];

fn spawn_level_transition_menu(world: &mut World) {
    let activate_id = world.register_system(activate);
    let deactivate_id = world.register_system(deactivate);
    let pressed_system_id = world.register_system(process_upgrade_and_go_to_next_level);

    let upgrades = world
        .resource::<SelectionsPool<GlobalUpgrade>>()
        .get_multiple_random(3);
    let asset_server = world.resource::<AssetServer>();
    let upgrades: Vec<Option<(GlobalUpgrade, Handle<Image>)>> = upgrades
        .iter()
        .map(|upgrade| {
            upgrade.clone().map(|upgrade| {
                let texture = upgrade.upgrade.icon_texture;
                (upgrade, asset_server.load(texture).clone())
            })
        })
        .collect();

    let outer_menu = world
        .spawn((
            NodeBundle {
                z_index: z_index::POPUP_MENU,
                style: Style {
                    // fill the entire window
                    height: Val::Percent(100.),
                    width: Val::Percent(100.),
                    ..Default::default()
                },
                ..Default::default()
            },
            UpgradeSelectMenuRoot,
            Name::new("UpgradeSelectMenu"),
        ))
        .id();
    // spawn the key
    let menu_body = world
        .spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Percent(5.)),
                    width: Val::Percent(100.),
                    height: Val::Auto,
                    flex_direction: FlexDirection::Row,
                    display: Display::Flex,
                    align_content: AlignContent::SpaceEvenly,

                    ..Default::default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..Default::default()
            },
            MultiChoiceParent {
                selected: Index::new(MENU_ITEMS.len(), 0),
            },
            UpgradeSelectMenu,
            Name::new("UpgradeSelectMenu"),
        ))
        .id();

    for (i, upgrade) in upgrades.iter().enumerate() {
        let option = world
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        margin: UiRect::all(Val::Percent(3.)),
                        border: UiRect::all(Val::Percent(1.)),
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    background_color: Color::GRAY.into(),
                    border_color: Color::WHITE.into(),
                    ..Default::default()
                },
                MultiChoiceButton {
                    on_selected: Some(pressed_system_id),
                    activate: activate_id,
                    deactivate: deactivate_id,
                },
                UpgradeOption(upgrade.as_ref().map(|(upgrade, _)| (*upgrade).clone())),
                Name::new(format!("Upgrade {i}")),
            ))
            .id();

        let header = world
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(20.),
                        display: Display::Flex,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                Name::new("Header"),
            ))
            .id();

        let body = world
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(80.),
                        display: Display::Flex,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                Name::new("Body"),
            ))
            .id();

        let title = world
            .spawn((
                TextBundle {
                    text: Text::from_section(
                        match upgrade {
                            None => "Placeholder",
                            Some((upgrade, _)) => upgrade.upgrade.name,
                        },
                        TextStyle {
                            font_size: 64.,
                            ..default()
                        },
                    )
                    .with_justify(JustifyText::Center),
                    style: Style { ..default() },
                    ..default()
                },
                Name::new("Header"),
            ))
            .id();
        let description_div = world
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::End,
                    height: Val::Percent(40.),
                    padding: UiRect::new(
                        // Val::Percent(10.),
                        // Val::Percent(10.),
                        Val::Percent(0.),
                        Val::Percent(0.),
                        Val::Percent(0.),
                        Val::Percent(10.),
                    ),
                    ..Default::default()
                },
                ..Default::default()
            })
            .id();

        // BUG: I think that if I have a div (flexbox, idk if related), inside of which there is a text node, and the text-node is multiline, it will ignore right-padding
        // Wrapping the text in description_div fixed it (TODO: open an issue)
        let mut description = world.spawn((
            TextBundle {
                style: Style {
                    ..Default::default()
                },
                ..default()
            },
            Name::new("Description"),
        ));
        if let Some((upgrade, _)) = upgrade {
            let mut text = description.get_mut::<Text>().unwrap();
            *text = Text::from_section(
                upgrade.upgrade.description,
                TextStyle {
                    font_size: 32.,
                    ..Default::default()
                },
            )
            .with_justify(JustifyText::Center);
        };
        let description = description.id();

        let mut icon = world.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Percent(5.),
                    width: Val::Percent(50.),
                    height: Val::Px(200.),
                    border: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                ..default()
            },
            Name::new("Image"),
        ));

        if let Some((_, icon_path)) = upgrade {
            icon.insert(UiImage::new((*icon_path).clone()));
            icon.get_mut::<BackgroundColor>().unwrap().0 = Color::WHITE;
        }
        let icon = icon.id();

        world.entity_mut(option).add_child(header);
        world.entity_mut(header).add_child(title);
        world.entity_mut(option).add_child(body);
        world.entity_mut(body).add_child(description_div);
        world.entity_mut(description_div).add_child(description);
        world.entity_mut(body).add_child(icon);

        world.entity_mut(menu_body).add_child(option);
    }
    world.entity_mut(outer_menu).add_child(menu_body);
}

fn despawn_level_transition_menu(
    mut commands: Commands,
    q_menu: Query<Entity, With<UpgradeSelectMenuRoot>>,
) {
    let menu = q_menu.get_single().unwrap();
    commands.entity(menu).despawn_recursive();
}

fn register_menu(
    menu_query: Query<Entity, (With<MultiChoiceParent>, With<UpgradeSelectMenu>)>,
    mut menu_stack: ResMut<MenuStack>,
    mut writer: EventWriter<SpawnedMenu>,
) {
    let new_menu = menu_query.get_single().unwrap();
    menu_stack.push_menu(new_menu);
    writer.send(SpawnedMenu(new_menu));
}

fn unregister_menu(
    menu_query: Query<Entity, (With<MultiChoiceParent>, With<UpgradeSelectMenu>)>,
    mut menu_stack: ResMut<MenuStack>,
) {
    let menu = menu_query.get_single().unwrap();
    menu_stack.pop_menu(menu);
}

pub fn deactivate(In(entity): In<Entity>, mut border_query: Query<&mut BorderColor>) {
    border_query.get_mut(entity).unwrap().0 = Color::DARK_GRAY;
}

pub fn activate(In(entity): In<Entity>, mut border_query: Query<&mut BorderColor>) {
    border_query.get_mut(entity).unwrap().0 = Color::WHITE;
}

fn process_upgrade_and_go_to_next_level(
    In(menu_item_entity): In<Entity>,
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
    upgrade_applier: Res<UpgradeApplier>,
    q_upgrade: Query<&UpgradeOption>,
) {
    if let Some(upgrade) = q_upgrade.get(menu_item_entity).unwrap().0.clone() {
        commands.run_system_with_input(upgrade_applier.apply_upgrade_to_all, upgrade);
    }
    app_state.0 = Some(AppState::Defending);
}

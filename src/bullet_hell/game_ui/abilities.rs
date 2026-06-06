use std::path::Path;

use bevy::{ecs::entity::EntityHashSet, prelude::*};
use bevy_inspector_egui::prelude::*;

use crate::{
    bullet_hell::{
        abilities3::{AbilityHolder, OnAbilityChange},
        player::Player,
    },
    ui,
    utils::z_index,
};

pub fn plugin(app: &mut App) {
    app.add_systems(PostStartup, spawn_ability_icon)
        .add_systems(Update, render_cooldown);
}

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
struct AbilityCooldown;

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
struct IconImage;

#[derive(Component)]
#[relationship(relationship_target = AbilityIcons)]
struct IconOf(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = IconOf, linked_spawn)]
struct AbilityIcons(EntityHashSet);

fn spawn_ability_icon(
    player: Query<Entity, With<Player>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) -> Result {
    let player = player.single()?;
    commands.spawn((
        Node {
            width: Val::Px(100.),
            height: Val::Px(100.),
            border: UiRect::all(Val::Percent(1.)),
            position_type: PositionType::Absolute,
            left: Val::Percent(10.),
            bottom: Val::Percent(5.),
            justify_self: JustifySelf::Center,
            ..Default::default()
        },
        z_index::GAME_UI,
        BackgroundColor(ui::palette::GRAY),
        Name::new("Ability Icon"),
        // IconOf(player.single()?),
        children![
            (
                ImageNode::new(asset_server.load(Path::new("sprites/upgrades/sword_diamond.png"))),
                Name::new("Icon"),
                IconImage,
                // IconOf(player),
            ),
            (
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(0.),
                    // margin: UiRect::all(Val::Percent(5.)),
                    ..Default::default()
                },
                BackgroundColor(ui::palette::DARK_GRAY.with_alpha(0.5)),
                Name::new("Cooldown"),
                AbilityCooldown,
                IconOf(player),
            ),
        ],
    ));
    // commands.entity(player).observe(on_ability_change);

    Ok(())
}

// fn render_cooldown(mut query: Query<(&mut AbilityCooldown, &mut Node)>, time: Res<Time>) {
//     for (mut icon, mut node) in query.iter_mut() {
//         node.height = Val::Percent(100. * icon.cooldown.fraction_remaining());
//         icon.cooldown.tick(time.delta());
//     }
// }

// fn render_cooldown2(
//     player: Query<&AbilityHolder>,
//     parent: Query<(&IconOf, &Children)>,
//     mut cooldown: Query<&mut Node, With<AbilityCooldown>>,
// ) -> Result {
//     for (icon_of, children) in parent.iter() {
//         if let Ok(ability_holder) = player.get(icon_of.0) {
//             let mut iterator = cooldown.iter_many_mut(children);
//             let mut node = iterator.fetch_next().unwrap();
//             node.height = Val::Percent(100. * ability_holder.cooldown().fraction_remaining());

//             assert!(iterator.fetch_next().is_none());
//         }
//     }
//     Ok(())
// }

fn on_ability_change(
    trigger: Trigger<OnAbilityChange>,
    player: Query<(&AbilityHolder, &AbilityIcons)>,
    mut images_query: Query<&mut ImageNode, With<AbilityCooldown>>,
    asset_server: Res<AssetServer>,
) {
    for (ability_holder, icons) in player.iter() {
        // let foo: EntityHashSet = icons.
        for mut image in images_query.iter_many_unique_mut(icons.0.clone()) {
            *image = ImageNode::new(asset_server.load(ability_holder.ability().icon_texture));
        }
    }
}

fn render_cooldown(
    player: Query<&AbilityHolder>,
    mut cooldown: Query<(&mut Node, &IconOf), With<AbilityCooldown>>,
) -> Result {
    for (mut node, icon_of) in cooldown.iter_mut() {
        let ability_holder = player.get(icon_of.0)?;
        node.height = Val::Percent(100. * ability_holder.cooldown().fraction_remaining());
    }
    Ok(())
}

use bevy::prelude::*;
// use bevy::sprite::MaterialMesh2dBundle;
// use bevy_rapier2d::prelude::*;

use super::physics_layers;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_arena);
    }
}

fn spawn_arena(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let size = 100.;
    let border_width = 5.;
    let position_offset = size / 2. - border_width / 2.;
    commands.spawn((Name::new("Arena"), Transform::from_xyz(0., 10., 0.)));
    commands.spawn((
        Name::new("Arena.Floor"),
        Mesh2d(
            meshes
                .add(bevy::math::primitives::Rectangle::new(size, border_width).mesh())
                .into(),
        ),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
        Transform::from_xyz(0., -position_offset, 0.),
        // Collider::cuboid(size / 2., border_width / 2.),
        // CollisionGroups::new(physics_layers::WALLS, physics_layers::ALL),
    ));
    commands.spawn((
        Name::new("Arena.Left"),
        Mesh2d(
            meshes
                .add(bevy::math::primitives::Rectangle::new(border_width, size).mesh())
                .into(),
        ),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
        Transform::from_xyz(-position_offset, 0., 0.),
        // Collider::cuboid(border_width / 2., size / 2.),
        // CollisionGroups::new(physics_layers::WALLS, physics_layers::ALL),
    ));
    commands.spawn((
        Name::new("Arena.Right"),
        Mesh2d(
            meshes
                .add(bevy::math::primitives::Rectangle::new(border_width, size).mesh())
                .into(),
        ),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
        Transform::from_xyz(position_offset, 0., 0.),
        // Collider::cuboid(border_width / 2., size / 2.),
        // CollisionGroups::new(physics_layers::WALLS, physics_layers::ALL),
    ));
    commands.spawn((
        Name::new("Arena.Roof"),
        Mesh2d(
            meshes
                .add(bevy::math::primitives::Rectangle::new(size, border_width).mesh())
                .into(),
        ),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
        Transform::from_xyz(0., position_offset, 0.),
        // Collider::cuboid(size / 2., border_width / 2.),
        // CollisionGroups::new(physics_layers::WALLS, physics_layers::ALL),
    ));
}

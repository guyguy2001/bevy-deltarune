use bevy::{prelude::*, ui::UiSystem};

pub struct WorldUIPlugin;
impl Plugin for WorldUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, sync_world_ui_to_parent.before(UiSystem::Layout))
            .add_systems(PostStartup, error_if_world_ui_without_style);
    }
}

#[derive(Component)]
pub struct WorldUI {
    pub tracked_entity: Entity,
}

fn sync_world_ui_to_parent(
    mut world_ui_query: Query<(&mut Style, &WorldUI)>,
    transform_query: Query<&GlobalTransform>,
    camera_query: Query<(&Camera, &GlobalTransform), With<IsDefaultUiCamera>>,
) {
    let (camera, camera_transform) = camera_query.single();
    for (mut style, world_ui) in world_ui_query.iter_mut() {
        let parent_transform = transform_query.get(world_ui.tracked_entity).unwrap();
        let position = camera
            .world_to_viewport(camera_transform, parent_transform.translation())
            .unwrap();
        style.left = Val::Px(position.x);
        style.top = Val::Px(position.y);
    }
}

fn error_if_world_ui_without_style(query: Query<(Entity, &WorldUI), Without<Style>>) {
    for (entity, _) in query.iter() {
        error!(
            "ERROR! {:?} has a WorldUI component, but not a style component!",
            entity
        )
    }
}

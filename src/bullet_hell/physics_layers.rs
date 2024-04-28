use bevy_rapier2d::geometry::Group;

pub const PLAYERS: Group = Group::GROUP_1;
pub const BULLETS: Group = Group::GROUP_2;
pub const WALLS: Group = Group::GROUP_3;
pub const ALL: Group = Group::ALL;
#[allow(unused)]
pub const NONE: Group = Group::NONE;

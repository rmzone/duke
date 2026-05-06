use bevy::app::App;
use bevy::prelude::{Component, Plugin};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

#[derive(Component)]
pub struct Player;
// state
// direction
// ...

// TODO: update animation_state based on direction, and state (idle, ....)
// make generic enough for a player or monster?

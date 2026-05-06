use std::collections::HashMap;
use bevy::app::App;
use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SpritesheetAnimationPlugin)
            .add_systems(Update, update_animation_state);
    }
}

#[derive(Component, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum State {
    Idle,
    Walking,
    Attacking(u32),
    Dashing,
    Dying,
}

#[derive(Component, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Component, PartialEq, Debug, Clone)]
pub struct Animations(pub HashMap<String, Handle<Animation>>);

#[derive(Component, PartialEq, Debug, Clone)]
pub struct DirectionalAnimations(pub HashMap<State, HashMap<Direction, Handle<Animation>>>);

fn update_animation_state(mut query: Query<(
    &DirectionalAnimations,
    &mut SpritesheetAnimation,
    &State,
    &Direction,
)>,) {
    // for (animations, mut spritesheet, state, direction) in query.iter_mut() {
    //     if let Some(state_animations) = animations.0.get(state) {
    //         if let Some(anim) = state_animations.get(direction) {
    //             if spritesheet.animation != *anim {
    //                 spritesheet.switch(*anim);
    //             }
    //         }
    //     }
    // }
}

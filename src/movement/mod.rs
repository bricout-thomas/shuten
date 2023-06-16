use bevy::prelude::*;
use crate::AppState;

// This crate specifies trajectories for bullets and enemies
// One might add multiple flight components and they should add up
// As they all add something to the translation each frame,
// except for spline defined trajectories

// one can add two time the same trajectory or a spline to another trajectory using a parent node

mod flights;
mod screenborders;

pub use flights::*;
pub use screenborders::*;

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                (
                move_circle_flight,
                move_linear_flight,
                ease_out_sine_flight,
                constrain_on_screen
                )
                    .in_set(OnUpdate(AppState::InGame)))

            .add_system(destroy_on_screen_left)
            .add_system(destroy_on_up)

            .register_type::<CircleFlight>()
            .register_type::<LinearFlight>()
            .register_type::<EaseOutSineFlight>()
            .register_type::<ConstrainOnScreen>()
            ;
    }
}

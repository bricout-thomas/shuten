use bevy::prelude::*;

// The enemies module handles enemy specific behaviors
// As well as spawn_[enemy] functions
mod minideathstar;
pub use minideathstar::spawn_death_star;

pub struct EnemyBehaviorPlugin;
impl Plugin for EnemyBehaviorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(minideathstar::look_at_player)
            ;
    }
}

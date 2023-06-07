use bevy::prelude::*;

// This module handles emitters, that is, anything that spewts bullets

struct EmittersPlugin;
impl Plugin for EmittersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(simple_directed_emitter)
            ;
    }
}

#[derive(Component)]
struct SimpleDirectedEmitter {
    timer: Timer
}

use crate::player::Player;
fn simple_directed_emitter(
    mut commands: Commands,
    mut emitter_query: Query<(&Transform, &mut SimpleDirectedEmitter)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_translation = player_query.get_single().unwrap().translation.truncate();
    for (transform, mut emitter) in emitter_query.iter_mut() {

    }
    // TODO
}

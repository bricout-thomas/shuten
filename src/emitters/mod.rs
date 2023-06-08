use bevy::prelude::*;
use crate::{
    bullets::spawn_simple_aimed_linear_bullet,
    assets::LoadedAssets,
};

// This module handles emitters, that is, anything that spewts bullets

pub struct EmittersPlugin;
impl Plugin for EmittersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(simple_directed_emitter)
            ;
    }
}

#[derive(Component)]
pub struct SimpleDirectedEmitter {
    pub timer: Timer,
}

use crate::player::Player;
fn simple_directed_emitter(
    mut commands: Commands,
    mut emitter_query: Query<(&Transform, &mut SimpleDirectedEmitter)>,
    player_query: Query<&Transform, With<Player>>,
    loaded_assets: Res<LoadedAssets>,
    time: Res<Time>,
) {
    let player_translation = player_query.get_single().unwrap().translation.truncate();
    for (transform, mut emitter) in emitter_query.iter_mut() {
        emitter.timer.tick(time.delta());
        if emitter.timer.just_finished() {
            spawn_simple_aimed_linear_bullet(&mut commands, &loaded_assets, player_translation, transform.translation.truncate(), false);
        }
    }
    // TODO
}

use bevy::prelude::*;
use crate::{
    bullets::{spawn_simple_aimed_linear_bullet, spawn_fixed_linear_bullet},
    assets::LoadedAssets,
};

// This module handles emitters, that is, anything that spewts bullets

pub struct EmittersPlugin;
impl Plugin for EmittersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(simple_directed_emitter)
            .add_system(ring_arc_emitter)

            .register_type::<SimpleDirectedEmitter>()
            .register_type::<RingEmitter>()

            ;
    }
}

#[derive(Default, Component, Reflect)]
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
    if let Ok(player_transform) = player_query.get_single() {
        let player_translation = player_transform.translation.truncate();
        for (transform, mut emitter) in emitter_query.iter_mut() {
            emitter.timer.tick(time.delta());
            if emitter.timer.just_finished() {
                spawn_simple_aimed_linear_bullet(&mut commands, &loaded_assets, 
                    player_translation, transform.translation.truncate(), false, 50.);
            }
        }
    }
}

#[derive(Component, Reflect, Default)]
pub struct RingEmitter {
    pub timer: Timer,
    pub bullet_count: u8,
}

fn emit_ring_arc(
    commands: &mut Commands,
    loaded_assets: &Res<LoadedAssets>,
    position: Vec2,
    angle: f32, // in radians
    bullet_count: u8,
    angle_deviate: f32, // angle distance between each bullets
    speed: f32,
    directed: bool,
) {
    let left_most_angle = angle - angle_deviate * bullet_count as f32 / 2.;
    for i in 0..bullet_count {
        let bullet_angle = left_most_angle + i as f32 * angle_deviate;
        spawn_fixed_linear_bullet(commands, &loaded_assets, position, bullet_angle, speed, directed)
    }
}

fn ring_arc_emitter(
    mut commands: Commands,
    mut emitter_query: Query<(&Transform, &mut RingEmitter)>,
    loaded_assets: Res<LoadedAssets>,
    time: Res<Time>,
) {
    for (transform, mut emitter) in emitter_query.iter_mut() {
        emitter.timer.tick(time.delta());
        if emitter.timer.just_finished() {
            emit_ring_arc(&mut commands, &loaded_assets, transform.translation.truncate(), std::f32::consts::TAU*3./4., emitter.bullet_count, std::f32::consts::TAU/30., 30., true)
        }
    }
}

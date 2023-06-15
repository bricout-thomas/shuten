use bevy::prelude::*;
use crate::assets::LoadedAssets;
use crate::BULLET_LAYER;
use super::Player;

#[derive(Component)]
pub struct Missile {
    timer: Timer,
}
#[derive(Component)]
pub struct MissileLauncher {
    pub timer: Timer,
}

pub fn player_launch_missile(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<(&Transform, &mut MissileLauncher), With<Player>>,
    time: Res<Time>,
    loaded_assets: Res<LoadedAssets>,
) {
    let delta = time.delta();
    for (transform, mut launcher) in player_query.iter_mut() {
        launcher.timer.tick(delta);
        if launcher.timer.finished() && keys.just_pressed(KeyCode::W){
            launcher.timer.reset();
            commands.spawn( SpriteBundle {
                transform: Transform::from_translation(transform.translation.truncate().extend(BULLET_LAYER)),
                texture: loaded_assets.player_bullet.clone(),
                ..default()
            })
                .insert(Missile { timer: Timer::from_seconds(10., TimerMode::Once)})
            ;
        }
    }
}

pub fn missile_move(
    mut query: Query<(&mut Transform, &mut Missile)>,
    time: Res<Time>,
) {
    let delta = time.delta();
    for (mut transform, mut missile) in query.iter_mut() {
        missile.timer.tick(delta);
        let t = missile.timer.elapsed_secs() / 10.; // should never go over ten so t in [0, 1]
        transform.translation += Vec3::Y * (1. - (1. - t).powf(5.)) * 4.;
    }
}

use crate::enemies::PlayerKillable;
pub fn missile_explode(
    mut commands: Commands,
    missile_query: Query<(Entity, &Transform, &Missile)>,
    mut enemy_query: Query<(Entity, &Transform, &mut PlayerKillable)>,
) {
    for (missile_e, missile_t, missile) in missile_query.iter() {
        if missile.timer.finished() {
            commands.entity(missile_e).despawn_recursive();
            for (enemy_e, enemy_t, mut killable) in enemy_query.iter_mut() {
                
            }
        }
    }
}

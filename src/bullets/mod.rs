use bevy::prelude::*;

use crate::{assets::LoadedAssets, movement::*, BULLET_LAYER, collisions::DamageCircleHitbox};

pub fn spawn_simple_aimed_linear_bullet(
    commands: &mut Commands,
    loaded_assets: &Res<LoadedAssets>,
    target: Vec2,
    position: Vec2,
    directed: bool,
    speed: f32
) -> Entity {
    let rotation = match directed {
        true => Quat::from_rotation_arc_2d(Vec2::Y, target - position),
        false => Quat::IDENTITY,
    };

    commands.spawn(
        SpriteBundle {
            texture: loaded_assets.red_bullet.clone() ,
            transform: Transform { translation: position.extend(BULLET_LAYER), rotation, ..default() },
            ..default()
        }
    )
        .insert(LinearFlight::from_target(target, position, speed))
        .insert(DamageCircleHitbox { radius_squared: 5. })
        .insert(DestroyOnScreenLeft { hitbox: 5. })
        .insert(Name::new("RedBullet"))
    .id()
}

pub fn spawn_fixed_linear_bullet(
    commands: &mut Commands,
    loaded_assets: &Res<LoadedAssets>,
    position: Vec2,
    angle: f32, // in radians
    speed: f32,
    directed: bool,
) -> Entity {
    let rotation = match directed {
        true => Quat::from_rotation_z(angle),
        false => Quat::IDENTITY,
    };

    commands.spawn(
        SpriteBundle {
            texture: loaded_assets.red_bullet.clone() ,
            transform: Transform { translation: position.extend(BULLET_LAYER), rotation, ..default() },
            ..default()
        }
    )
        .insert(LinearFlight::from_angle(angle, speed))
        .insert(DamageCircleHitbox { radius_squared: 5. })
        .insert(DestroyOnScreenLeft { hitbox: 5. })
        .insert(Name::new("RedBullet"))
    .id()
}

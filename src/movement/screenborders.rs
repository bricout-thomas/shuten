use bevy::prelude::*;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, HALF_SCREEN_HEIGHT, HALF_SCREEN_WIDTH};

// insert this on bullets, as they should not surprise the player by appearing from out screen
#[derive(Component)]
pub struct DestroyOnScreenLeft {
    pub hitbox: f32,
}

pub fn destroy_on_screen_left(
    query: Query<(Entity, &Transform, &DestroyOnScreenLeft)>,
    mut commands: Commands,
) {
    for (entity, transform, des) in query.iter() {
        let position = transform.translation.truncate();
        let s = des.hitbox;
        if position.x+s < -SCREEN_WIDTH || position.x-s > SCREEN_WIDTH || position.y+s < -SCREEN_HEIGHT || position.x-s > SCREEN_HEIGHT {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// insert this on player generated projectiles, as they can only go up
// exept of course if they don't
#[derive(Component)]
pub struct DestroyOnUp {
    pub hitbox: f32,
}

pub fn destroy_on_up (
    query: Query<(Entity, &Transform, &DestroyOnUp)>,
    mut commands: Commands,
) {
    for (entity, transform, des) in query.iter() {
        if transform.translation.y - des.hitbox > HALF_SCREEN_HEIGHT {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// constrains ( probably only the player ) to the screen
#[derive(Component, Default, Reflect)]
pub struct ConstrainOnScreen { pub half_hitbox: f32 }

pub fn constrain_on_screen (
    mut query: Query<(&ConstrainOnScreen, &mut Transform)>,
) {
    for (constraint, mut transform) in query.iter_mut() {
        transform.translation.x = transform.translation.x.min(HALF_SCREEN_HEIGHT-constraint.half_hitbox).max(-HALF_SCREEN_HEIGHT+constraint.half_hitbox);
        transform.translation.y = transform.translation.y.min(HALF_SCREEN_WIDTH -constraint.half_hitbox).max(-HALF_SCREEN_WIDTH +constraint.half_hitbox);
    }
}

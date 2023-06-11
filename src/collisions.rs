use bevy::prelude::*;
use crate::{player::Player, AppState};

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.
            add_system(damage_circle_collision.in_set(OnUpdate(AppState::InGame)))
            ;
    }
}

#[derive(Component)]
pub struct DamageCircleHitbox {
    pub radius_squared: f32,
}

fn damage_circle_collision(
    mut player_query: Query<(&GlobalTransform, &mut Player)>,
    hitboxes_query: Query<(&GlobalTransform, &DamageCircleHitbox)>,
    time: Res<Time>,
) {
    if let Ok((player_transform, mut player)) = player_query.get_single_mut() {
        if player.invincibility <= 0. {
            let player_pos = player_transform.translation().truncate();
            for (transform, hitbox) in hitboxes_query.iter() {
                if player_pos.distance_squared(transform.translation().truncate()) < hitbox.radius_squared {
                    println!("player hit");
                    player.invincibility = 5.;
                }
            }
        } else {
            player.invincibility -= time.delta_seconds();
        }
    }
}


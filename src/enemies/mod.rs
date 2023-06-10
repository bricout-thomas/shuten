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
            .add_system(player_killable_hit)

            .register_type::<PlayerKillable>()
            ;
    }
}

#[derive(Component, Reflect, Default)]
struct PlayerKillable {
    health: u16,
    hitbox: f32,
}

use crate::player::PlayerBullet;
fn player_killable_hit (
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut PlayerKillable)>,
    player_bullet_query: Query<(Entity, &Transform), With<PlayerBullet>>
) {
    for (enemy_entity, enemy_transform, mut killable) in query.iter_mut() {
        for (bullet_entity, bullet_transform) in player_bullet_query.iter() {
            let bullet_pos = bullet_transform.translation.truncate();
            let enemy_pos = enemy_transform.translation.truncate();
            let hit =   bullet_pos.x > enemy_pos.x - killable.hitbox &&
                        bullet_pos.x < enemy_pos.x + killable.hitbox &&
                        bullet_pos.y > enemy_pos.y - killable.hitbox &&
                        bullet_pos.y < enemy_pos.y + killable.hitbox;

            if hit {
                commands.entity(bullet_entity).despawn_recursive();
                killable.health -= 1;
                if killable.health == 0 {
                    commands.entity(enemy_entity).despawn_recursive();
                }
            }
        }
    }
}

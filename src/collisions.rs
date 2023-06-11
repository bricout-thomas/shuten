use bevy::prelude::*;
use crate::{player::Player, AppState};

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(damage_circle_collision.in_set(OnUpdate(AppState::InGame)))
            .add_event::<PlayerHitEvent>()
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
    mut hit_ev: EventWriter<PlayerHitEvent>,
    mut appstate: ResMut<NextState<AppState>>,
    time: Res<Time>,
) {
    for (player_transform, mut player) in player_query.iter_mut() {
        if player.invincibility <= 0. {
            let player_pos = player_transform.translation().truncate();
            'dealer_loop: for (transform, hitbox) in hitboxes_query.iter() {
                if player_pos.distance_squared(transform.translation().truncate()) < hitbox.radius_squared {
                    player_hit_consequences(&mut player, &mut appstate, &mut hit_ev);
                    break 'dealer_loop; // prevents from trigerring twice
                }
            }
        } else {
            player.invincibility -= time.delta_seconds();
        }
    }
}

struct PlayerHitEvent;

fn player_hit_consequences(
    player: &mut Player,
    appstate: &mut ResMut<NextState<AppState>>,
    hit_ev: &mut EventWriter<PlayerHitEvent>,
) {
    player.invincibility = 5.;
    match player.health.checked_sub(1) {
        Some(health) => { player.health = health; hit_ev.send(PlayerHitEvent) },
        None => { appstate.set(AppState::Death); player.health = 0 }
    }
}

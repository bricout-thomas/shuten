use bevy::prelude::*;
use crate::emitters::SimpleDirectedEmitter;
use crate::movement::{EaseOutSineFlight, CircleFlight};

#[derive(Component)]
pub struct MiniDeathStar;

#[derive(Bundle)]
struct MiniDeathStarBundle {
    minideathstar: MiniDeathStar,
    #[bundle]
    sprite: SpriteSheetBundle,
}

use crate::assets::LoadedAssets;
pub fn spawn_death_star (
    commands: &mut Commands,
    loaded_assets: &Res<LoadedAssets>,
    position: Vec2,
) {
    let death_star_spritesheet = SpriteSheetBundle {
        texture_atlas: loaded_assets.minideathstar_spritesheet.clone(),
        transform: Transform::from_translation(position.extend(0.)),
        ..default()
    };
    commands.spawn(
        MiniDeathStarBundle {
            minideathstar: MiniDeathStar,
            sprite: death_star_spritesheet,
        }
    )
        .insert(CircleFlight { t: 0., amplitude: 10., angular_speed: 0.5, } )
        .insert(SimpleDirectedEmitter { timer: Timer::from_seconds(1., TimerMode::Repeating) })
        .insert(Name::new("MiniDeathStar"))
        .insert(EaseOutSineFlight { t: 0., path: Vec2::NEG_X*50., time: 2.})
    ;
}

use crate::player::Player;
pub fn look_at_player(
    player_query: Query<&Transform, With<Player>>,
    mut star_query: Query<(&Transform, &mut TextureAtlasSprite), With<MiniDeathStar>>
) {
    let player_translation = player_query.get_single().unwrap().translation.truncate();
    for (transform, mut star_sprite) in star_query.iter_mut() {
        let look_vector = player_translation - transform.translation.truncate();
        let look_angle = look_vector.angle_between(Vec2::Y); // in radians
        star_sprite.index = (look_angle * 18. / (2. * std::f32::consts::PI) + 9.).round() as usize;
    }
}

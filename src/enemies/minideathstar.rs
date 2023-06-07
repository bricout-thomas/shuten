use bevy::prelude::*;
use crate::movement::CircleFlight;

#[derive(Component)]
pub struct MiniDeathStar;

#[derive(Bundle)]
struct MiniDeathStarBundle {
    minideathstar: MiniDeathStar,
    #[bundle]
    sprite: SpriteSheetBundle,
}

pub fn spawn_death_star (
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    position: Vec2,
) {
    let death_star_spritesheet = SpriteSheetBundle {
        sprite: TextureAtlasSprite { index: 0, ..default() },
        texture_atlas: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("Minideathstar.png"),
            Vec2::new(17., 17.), 4, 5, None, None
        )),
        transform: Transform::from_translation(position.extend(0.)),
        ..default()
    };
    commands.spawn(
        MiniDeathStarBundle {
            minideathstar: MiniDeathStar,
            sprite: death_star_spritesheet,
        }
    ).insert(CircleFlight { t: 0., amplitude: 10., angular_speed: 0.5, } );
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

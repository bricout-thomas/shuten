use bevy::prelude::*;
use crate::{PLAYER_LAYER, assets::LoadedAssets, PLAYER_BULLET_LAYER, movement::{LinearFlight, DestroyOnUp}, AppState};

#[derive(Component)]
pub struct Player {
    rotation: f32,      // describes how much the sprite is turning to the right ( negative = left )
    pub invincibility: f32, // time left before it ends
}

#[derive(Bundle)]
struct PlayerBundle {
    p: Player,

    #[bundle]
    sprite: SpriteSheetBundle,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player)
            .add_system(move_player.in_set(OnUpdate(AppState::InGame)))
            .add_system(player_shoot.in_set(OnUpdate(AppState::InGame)))
        ;
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let player_sprite_sheet = SpriteSheetBundle {
        sprite: TextureAtlasSprite { index: 0, ..default() },
        texture_atlas: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load("Ship.png"),
            Vec2::new(15., 15.), 2, 3, None, None,
        )),
        transform: Transform::from_translation(Vec3::Z * PLAYER_LAYER),
        ..default()
    };

    commands.spawn(
        PlayerBundle {
            p: Player { rotation: 0., invincibility: 0. },
            sprite: player_sprite_sheet,
        }
    )
        .insert(NormalWeapon { timer: Timer::from_seconds(0.1, TimerMode::Repeating) })
        .insert(Name::new("Player"))
    ;
}

fn move_player(
    mut player_query: Query<(&mut Player, &mut Transform, &mut TextureAtlasSprite)>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
) {
    let (mut player, mut transform, mut atlas_sprite) = player_query.get_single_mut().unwrap();

    let delta = time.delta_seconds();
    let slower = keys.pressed(KeyCode::LShift);
    let max_rotation: f32 = if slower { 0.6 } else { 1.0 };
    let player_speed = if slower { 20. } else { 80. };
    let travelled_distance = player_speed * delta;

    let vertical = match (keys.pressed(KeyCode::Up),keys.pressed(KeyCode::Down)) {
        (true, false) => 1.,
        (false, true) => -1.,
        _ => 0.
    };
    let horizontal = match (keys.pressed(KeyCode::Right),keys.pressed(KeyCode::Left)) {
        (true, false) => { player.rotation = (max_rotation).min(player.rotation + delta*5.) ; 1. },
        (false, true) => { player.rotation = (-max_rotation).max(player.rotation - delta*5.) ; -1. },
        _ => { player.rotation += if player.rotation > 0. { -delta*2. } else { delta*2. } ; 0.}
    };

    let diagnal = 2_f32.sqrt().recip();
    let velocity = if vertical == 0. {
        Vec2::new(horizontal * travelled_distance, 0.)
    } else if horizontal == 0. {
        Vec2::new(0., vertical * travelled_distance)
    } else {
        Vec2::new(horizontal, vertical) * diagnal * travelled_distance
    };

    transform.translation += velocity.extend(0.);

    // animate the ship
    atlas_sprite.index = match player.rotation {
        x if x < -0.8 => 4,
        x if x < -0.2 => 3,
        x if x <= 0.2 => 0,
        x if x <= 0.8 => 1,
        _ => 2,
    }
}

#[derive(Component, Reflect, Default)]
struct NormalWeapon {
    timer: Timer,
}

#[derive(Component)]
pub struct PlayerBullet;

fn player_shoot(
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut NormalWeapon), With<Player>>,
    loaded_assets: Res<LoadedAssets>,
    time: Res<Time>,
) {
    if !keys.pressed(KeyCode::W) { return; }
    for (transform, mut weapon) in player_query.iter_mut() {
        weapon.timer.tick(time.delta());
        if weapon.timer.just_finished() {
            let bullet_amount = 5;
            let bullet_distance = 5.;
            for i in 0..bullet_amount {
                let position = 
                    (transform.translation.truncate() + Vec2::X * (- bullet_amount as f32 * bullet_distance / 2. + i as f32 * bullet_distance))
                    .extend(PLAYER_BULLET_LAYER);
                commands.spawn(
                    SpriteBundle {
                        texture: loaded_assets.player_bullet.clone(),
                        transform: Transform::from_translation(position),
                        ..default()
                    }
                )
                    .insert( LinearFlight::from_angle(std::f32::consts::FRAC_PI_2, 200.) )
                    .insert( DestroyOnUp { hitbox: 20. } )
                    .insert( PlayerBullet )
                    .insert( Name::new("PlayerBullet") )
                ;
            }
        }
    }
}

#[derive(Component, Reflect, Default)]
struct NormalWeapon {
    timer: Timer,
}


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

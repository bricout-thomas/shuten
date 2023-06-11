use bevy::prelude::*;

use crate::{UI_LAYER, SCREEN_WIDTH, SCREEN_HEIGHT, player::Player, assets::LoadedAssets, HALF_SCREEN_WIDTH, HALF_SCREEN_HEIGHT};

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(health_status_spawn)
            .add_system(health_status_update)
        ;
    }
}

#[derive(Component)]
struct HealthStatusUI {
    health: u8,
}

fn health_status_spawn (
    mut commands: Commands,
    loaded_assets: Res<LoadedAssets>,
) {
    // let player = player_query.get_single().unwrap();
    let health = 5;
    let health_ui = commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(150., 110., UI_LAYER),
        ..default()
    })
        .insert(HealthStatusUI { health })
        .insert(Name::new("HealthStatusUI"))
        .id();

    for i in 0..health {
        let new_heart = commands.spawn( SpriteBundle {
            transform: Transform::from_translation(Vec3::NEG_Y * i as f32 * 14.),
            texture: loaded_assets.health_indicator.clone(),
            ..default()
        }).id();
        
        commands.entity(health_ui).add_child(new_heart);
    }
}

fn health_status_update (
    mut commands: Commands,
    mut ui_query: Query<(Entity, &mut HealthStatusUI)>,
    player_query: Query<&Player, Changed<Player>>,
    loaded_assets: Res<LoadedAssets>,
) {
    let (ui_e, mut health_status) = ui_query.get_single_mut().unwrap();
    let Ok(player) = player_query.get_single() else {return;};
    let health = player.health;
    commands.entity(ui_e).despawn_descendants();
    for i in 0..health {
        let new_heart = commands.spawn( SpriteBundle {
            transform: Transform::from_translation(Vec3::NEG_Y * i as f32 * 14.),
            texture: loaded_assets.health_indicator.clone(),
            ..default()
        }).id();
        
        commands.entity(ui_e).add_child(new_heart);
    }
}

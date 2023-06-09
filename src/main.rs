use bevy::prelude::*;
use bevy::window::{WindowResolution, WindowMode};
use bevy_pixel_camera::{PixelCameraPlugin, PixelCameraBundle, PixelBorderPlugin};
use movement::MovementPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod player;
mod enemies;
mod movement;
mod testlevel;
mod emitters;
mod bullets;
mod collisions;
mod assets;

fn main() {
    App::new()
        // window setup
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some( Window {
                title: "shuten".into(),
                resolution: WindowResolution::new( 640., 480. ),
                resizable: true,
                mode: WindowMode::Windowed,
                ..default()
            }),
            .. default()
        }).set(ImagePlugin::default_nearest()))
        .add_plugin(PixelCameraPlugin)
        .add_plugin(PixelBorderPlugin {
            color: Color::rgb(0.1, 0.1, 0.1),
        })
        .add_startup_system(setup)
        .init_resource::<assets::LoadedAssets>()

        .add_plugin(player::PlayerPlugin)
        .add_plugin(MovementPlugin) // moves enemies and bullets
        .add_plugin(enemies::EnemyBehaviorPlugin) // enemy specific behaviors defined in the enemies module
        .add_plugin(emitters::EmittersPlugin)
        .add_plugin(collisions::CollisionPlugin)
        

        // tests
        .add_plugin(testlevel::TestLevelPlugin)

        // bevy_inspector_equi
        .add_plugin(WorldInspectorPlugin::new())

        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn( PixelCameraBundle::from_resolution(SCREEN_HEIGHT_i32,SCREEN_WIDTH_i32));
}

// The screen size ( pixelated resolution for camera and engine transform reference )
const SCREEN_HEIGHT_i32: i32 = 320;
const SCREEN_WIDTH_i32: i32 = 240;
const SCREEN_HEIGHT: f32 = 320.;
const SCREEN_WIDTH: f32 = 240.;

// the z value of diferent elements on screen
const BULLET_LAYER: f32 = 5.;
const PLAYER_LAYER: f32 = 4.;

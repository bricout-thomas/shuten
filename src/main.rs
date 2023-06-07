use bevy::prelude::*;
use bevy::window::{WindowResolution, WindowMode};
use bevy_pixel_camera::{PixelCameraPlugin, PixelCameraBundle, PixelBorderPlugin};
use movement::MovementPlugin;

mod player;
mod enemies;
mod movement;
mod testlevel;
mod emitters;

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
        .add_plugin(player::PlayerPlugin)
        .add_plugin(MovementPlugin) // moves enemies and bullets
        .add_plugin(enemies::EnemyBehaviorPlugin) // enemy specific behaviors defined in the enemies module
        

        // tests
        .add_plugin(testlevel::TestLevelPlugin)

        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn( PixelCameraBundle::from_resolution(320, 240) );
}

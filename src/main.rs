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
mod ui;
mod background;

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
        .insert_resource(ClearColor(Color::rgb_u8(96, 103, 130)))
        .add_plugin(PixelCameraPlugin)
        .add_plugin(PixelBorderPlugin {
            color: Color::rgb(0.1, 0.1, 0.1),
        })
        .add_startup_system(setup)
        .init_resource::<assets::LoadedAssets>()
        .add_state::<AppState>()

        .add_plugin(player::PlayerPlugin)
        .add_plugin(MovementPlugin)                 // moves enemies and bullets
        .add_plugin(enemies::EnemyBehaviorPlugin)   // enemy specific behaviors defined in the enemies module
        .add_plugin(emitters::EmittersPlugin)       // emits bullets, defines their spawning and
        .add_plugin(collisions::CollisionPlugin)    // collision between enemies, player and en
        .add_plugin(ui::UIPlugin)
        .add_plugin(background::BackgroundPlugin)

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

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
enum AppState {
    #[default]
    InGame,
    Paused,
    Death,
}

// The screen size ( pixelated resolution for camera and engine transform reference )
#[allow(non_upper_case_globals)]
const SCREEN_HEIGHT_i32: i32 = 320;
#[allow(non_upper_case_globals)]
const SCREEN_WIDTH_i32: i32 = 240;
const SCREEN_HEIGHT: f32 = 320.;
const SCREEN_WIDTH: f32 = 240.;
const HALF_SCREEN_HEIGHT: f32 = SCREEN_HEIGHT/2.;
const HALF_SCREEN_WIDTH: f32 =   SCREEN_WIDTH/2.;


// the z value of diferent elements on screen
const BULLET_LAYER: f32 = 5.;
const PLAYER_BULLET_LAYER: f32 = 3.;
const PLAYER_LAYER: f32 = 4.;
const UI_LAYER: f32 = 4.5;

use bevy::prelude::*;
use crate::{enemies::spawn_death_star, assets::LoadedAssets, SCREEN_HEIGHT, emitters::*, AppState, HALF_SCREEN_WIDTH, SCREEN_WIDTH};

// small test level to test touhou like boss spell cards

pub struct TestLevelPlugin;
impl Plugin for TestLevelPlugin {
    fn build(&self, app: &mut App){
        app
            .add_startup_system(spawn_test_ministars)
            ;
    }
}

fn spawn_test_ministars(
    mut commands: Commands,
    loaded_assets: Res<LoadedAssets>,
) {
    let death_star = spawn_death_star(&mut commands, &loaded_assets, Vec2::new(0., SCREEN_HEIGHT/2.+20.));
    commands.entity(death_star)
        .insert(RingEmitter { timer: Timer::from_seconds(0.1, TimerMode::Repeating), bullet_count: 8, } )
        // .insert(RandomBSplineFlight::new(default()))
    ;
}


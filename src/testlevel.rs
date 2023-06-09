use bevy::prelude::*;
use crate::{enemies::spawn_death_star, assets::LoadedAssets, SCREEN_HEIGHT};

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
    spawn_death_star(&mut commands, &loaded_assets, Vec2::new(0., SCREEN_HEIGHT/2.+20.));
}

use bevy::prelude::*;
use crate::{enemies::spawn_death_star, assets::LoadedAssets};

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
    for i in 0..5 {
        spawn_death_star(&mut commands, &loaded_assets, Vec2::new(0., 10.*i as f32));
    }
}

use bevy::prelude::*;
use crate::enemies::spawn_death_star;

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
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for i in 0..5 {
        spawn_death_star(&mut commands, &asset_server, &mut texture_atlases, Vec2::new(0., 10.*i as f32));
    }
}

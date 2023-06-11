use bevy::prelude::*;

// This modules help load assets so that one doesn't have to pass around
// AssetServer and TextureAtlases all the time
// assets are stored in LoadedAssets resource

#[derive(Resource)]
pub struct LoadedAssets {
    pub minideathstar_spritesheet: Handle<TextureAtlas>,
    pub red_bullet: Handle<Image>,
    pub player_bullet: Handle<Image>,
    pub health_indicator: Handle<Image>,
}

impl FromWorld for LoadedAssets {
    fn from_world(world: &mut World) -> Self {

    // This structure is necessary so as not to borrow mutably world twice at the same time

    // define texture atlases
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let minideathstar = TextureAtlas::from_grid(    
        asset_server.load("Minideathstar.png"),
        Vec2::new(17., 17.), 4, 5, None, None);
        drop(asset_server);
    // insert texture atlases and get their handles
        let mut texture_atlases = world.get_resource_mut::<Assets<TextureAtlas>>().unwrap();
        let minideathstar_spritesheet = texture_atlases.add(minideathstar);
        drop(texture_atlases);
    // create loaded assets
    let asset_server = world.get_resource::<AssetServer>().unwrap();
    LoadedAssets {
        minideathstar_spritesheet,
        red_bullet: asset_server.load("bullet.png"),
        player_bullet: asset_server.load("player_bullet.png"),
        health_indicator: asset_server.load("health.png")
    }
    }
}

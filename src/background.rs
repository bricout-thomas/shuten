use bevy::prelude::*;
use bevy::sprite::{Mesh2dHandle, MaterialMesh2dBundle, Material2dPlugin, Material2d};
use bevy::render::render_resource::{ShaderRef, AsBindGroup};
use bevy::reflect::{TypeUuid, Uuid};


pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(Material2dPlugin::<CustomBackgroundMaterial>::default())
            .add_startup_system(spawn_background)
        ;
    }
}

fn spawn_background(
    mut commands: Commands,
    mut materials: ResMut<Assets<CustomBackgroundMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn( MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Mesh::from(shape::Plane::from_size(250.)))),
        material: materials.add(CustomBackgroundMaterial { ..default() } ),
        ..default()
    })
        .insert(Name::new("Background"))
    ;
}

#[derive(Component)]
struct Background;

#[derive(AsBindGroup, TypeUuid, Debug, Clone, Default)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
struct CustomBackgroundMaterial {
    // Uniform bindings must implement `ShaderType`, which will be used to convert the value to
    // its shader-compatible equivalent. Most core math types already implement `ShaderType`.
    #[uniform(0)]
    color: Color,
    // Images can be bound as textures in shaders. If the Image's sampler is also needed, just
    // add the sampler attribute with a different binding index.
    #[texture(1)]
    #[sampler(2)]
    color_texture: Handle<Image>,
}

impl Material2d for CustomBackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/background.wgsl".into()
    }
}
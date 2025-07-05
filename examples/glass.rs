use bevy::{
    asset::load_internal_asset,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};

const SHADER_ASSET_PATH: &str = "glass_shader.wgsl";

fn main() {
    let mut app = App::new();



    app.add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<GlassMaterial>::default())
        .add_systems(Startup, setup)
        .run();
}

// Custom material definition
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct GlassMaterial {
    #[uniform(0)] // Corresponds to @group(1) @binding(0) in the shader
    color: LinearRgba,
}

impl Material for GlassMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend 
    }
}

// Setup system to create the scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GlassMaterial>>,
    mut materials_standard: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a cube with the glass material
    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::default())),
    //     MeshMaterial3d(materials.add(GlassMaterial {
    //         // Slightly blue, mostly white, and transparent
    //         color: LinearRgba::new(0.85, 0.9, 1.0, 0.001),
    //     })),
    //     Transform::from_xyz(0.0, 0.5, 0.0),
    // ));


    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials_standard.add(StandardMaterial{
            base_color: Color::srgba(0.85, 0.9, 1.0, 0.15), // Slightly blue, mostly white, with alpha for transparency
            alpha_mode: AlphaMode::Blend,                  // Enable blending for transparency
            perceptual_roughness: 0.08,                    // Smooth surface for glass
            metallic: 0.0,                                 // Non-metallic
            reflectance: 0.4,                              // Moderate reflectance
            diffuse_transmission: 0.7,                     // Allow light to pass through diffusely (key for glass)
            specular_transmission: 0.6,                    // Allow light to pass through specularly
            thickness: 1.0,                                // Thickness can affect transmission
            ior: 1.5,                                      // Index of Refraction for glass
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials_standard.add(StandardMaterial{
            base_color: Color::srgb(0., 1., 0.),
            ..Default::default()
        })),
        Transform::from_xyz(0.4, 0.3, -2.0).with_scale(Vec3::splat(0.5)),
    ));

    // // Spawn a ground plane (optional, for context)
    // commands.spawn((
    //     Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
    //     MeshMaterial3d(materials.add(GlassMaterial {
    //         // Using glass for the plane too for simplicity
    //         color: LinearRgba::new(0.7, 0.7, 0.8, 0.5), // Darker, more opaque glass for plane
    //     })),
    //     Transform::from_xyz(0.0, 0.5, 0.0),
    // ));

    // Spawn a point light
    commands.spawn((
        PointLight {
            intensity: 1500.0, // Adjust intensity as needed
            // shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Spawn a camera
    commands.spawn((
        Camera::default(),
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

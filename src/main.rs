use bevy::{
    asset::AssetApp,
    prelude::*,
    reflect::TypePath,
    render::camera::ScalingMode,
    render::render_resource::{AsBindGroup, ShaderRef},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<FresnelMaterial>::default())
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FresnelMaterial>>,
) {
    // sphere with fresnel material
    commands.spawn((
        Mesh3d(meshes.add(Sphere::default())),
        MeshMaterial3d(materials.add(FresnelMaterial {
            color: LinearRgba::BLACK,
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // camera
    commands.spawn((
        Camera::default(),
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 20.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct FresnelMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material for FresnelMaterial {


    fn fragment_shader() -> ShaderRef {
        "fresnel_shader.wgsl".into()
    }
}


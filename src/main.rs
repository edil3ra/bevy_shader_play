use bevy::{
    asset::load_internal_asset,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};

// Handle for the embedded shader
const GLASS_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(0x8A2F4B6C_E1D0_459F_8765_A3C2D1B8E0F1);

// The WGSL shader code as a string
const GLASS_SHADER_CODE: &str = r#"
// Import necessary Bevy PBR bindings and functions
#import bevy_pbr::mesh_functions::{mesh_position_local_to_world}
#import bevy_pbr::pbr_bindings::{globals, mesh_view_bindings}

// Custom material uniform struct
// Matches the GlassMaterial struct in Rust
@group(1) @binding(0)
var<uniform> material_properties: vec4<f32>; // color (r, g, b, a)

// Vertex shader input structure
struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>, // Included for completeness, not used in this simple shader
    @location(2) uv: vec2<f32>,
};

// Vertex shader output structure
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vertex(vertex_input: Vertex) -> VertexOutput {
    var out: VertexOutput;
    // Calculate the world position of the vertex
    let world_position = mesh_position_local_to_world(mesh_view_bindings.model, vec4<f32>(vertex_input.position, 1.0));
    // Transform to clip space
    out.clip_position = globals.view_proj * world_position;
    out.uv = vertex_input.uv;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var base_color = material_properties;

    // Calculate distance from the center of the UV map (0.0 at center, 1.0 at edges/corners)
    let dist_from_center_x = abs(in.uv.x - 0.5) * 2.0;
    let dist_from_center_y = abs(in.uv.y - 0.5) * 2.0;
    let dist_from_center = max(dist_from_center_x, dist_from_center_y);

    // Define border properties
    let border_width = 0.1; // How thick the border effect is (0.0 to 1.0 relative to dist_from_center)
    // Calculate border_factor: 0.0 in the central area, ramps up to 1.0 at the border
    let border_factor = smoothstep(1.0 - border_width, 1.0, dist_from_center);

    // Define the color to mix towards at the border (pure white)
    let border_highlight_color = vec3<f32>(1.0, 1.0, 1.0);
    // Strength of the whitening effect at the border
    let border_highlight_strength = 0.2; 

    // Mix the base color's RGB with the border highlight color
    let final_rgb = mix(base_color.rgb, border_highlight_color, border_factor * border_highlight_strength);

    return vec4<f32>(final_rgb, base_color.a);
}
"#;

fn main() {
    let mut app = App::new();

    // Load the embedded shader
    load_internal_asset!(
        app,
        GLASS_SHADER_HANDLE,
        "glass_shader.wgsl", 
        Shader::from_wgsl
    );

    app.add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<GlassMaterial>::default())
        .add_systems(Startup, setup)
        .run();
}

// Custom material definition
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct GlassMaterial {
    #[uniform(0)] // Corresponds to @group(1) @binding(0) in the shader
    color: Color,
}

impl Material for GlassMaterial {
    fn vertex_shader() -> ShaderRef {
        GLASS_SHADER_HANDLE.into()
    }

    fn fragment_shader() -> ShaderRef {
        GLASS_SHADER_HANDLE.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend // Enable transparency
    }
}

// Setup system to create the scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GlassMaterial>>,
) {
    // Spawn a cube with the glass material
    commands.spawn(MaterialMesh {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(GlassMaterial {
            // Slightly blue, mostly white, and transparent
            color: Color::rgba(0.85, 0.9, 1.0, 0.15),
        }),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    // Spawn a ground plane (optional, for context)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
        material: materials.add(GlassMaterial {
            // Using glass for the plane too for simplicity
            color: Color::rgba(0.7, 0.7, 0.8, 0.5), // Darker, more opaque glass for plane
        }),
        ..default()
    });

    // Spawn a point light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0, // Adjust intensity as needed
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Spawn a camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

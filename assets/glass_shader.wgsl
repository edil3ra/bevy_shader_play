// Import necessary Bevy PBR bindings and functions (Bevy 0.16 style)
#import bevy_pbr::mesh_bindings::mesh
#import bevy_pbr::view_bindings::view
#import bevy_pbr::mesh_functions::{mesh_vertex_position_local_to_world, mesh_normal_local_to_world}

// Custom material uniform struct
// Matches the GlassMaterial struct in Rust
// Should be @group(1) for user materials with AsBindGroup derive
@group(1) @binding(0)
var<uniform> material_properties: vec4<f32>; // color (r, g, b, a)

// Vertex shader input structure
struct Vertex {
    // instance_index is available if needed, but not used in this version
    // @builtin(instance_index) instance_index: u32, 
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

// Vertex shader output structure
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) world_position: vec4<f32>, // Pass world position
    @location(2) world_normal: vec3<f32>,   // Pass world normal
};

@vertex
fn vertex(vertex_input: Vertex) -> VertexOutput {
    var out: VertexOutput;

    // Calculate world position
    out.world_position = mesh_vertex_position_local_to_world(mesh.model, vertex_input.position);
    
    // Calculate clip position
    out.clip_position = view.view_proj * out.world_position;
    
    // Transform normal to world space
    out.world_normal = mesh_normal_local_to_world(vertex_input.normal, mesh.model);
    
    out.uv = vertex_input.uv;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // --- Border Effect ---
    let dist_from_center_x = abs(in.uv.x - 0.5) * 2.0;
    let dist_from_center_y = abs(in.uv.y - 0.5) * 2.0;
    let dist_from_center = max(dist_from_center_x, dist_from_center_y);

    let border_width = 0.05; 
    let border_factor = smoothstep(1.0 - border_width, 1.0, dist_from_center);
    
    // Changed border highlight to white and made it more subtle
    let border_highlight_color = vec3<f32>(1.0, 1.0, 1.0); 
    let border_highlight_strength = 0.2;

    // --- Fresnel Effect ---
    let normal = normalize(in.world_normal);
    // Ensure view.world_position is vec3 for subtraction with in.world_position.xyz
    let view_dir = normalize(view.world_position.xyz - in.world_position.xyz);
    
    // NdotV: Dot product between normal and view direction.
    // Clamped to [0, 1] for stability with pow.
    let NdotV = clamp(dot(normal, view_dir), 0.0, 1.0);
    
    // fresnel_power controls the sharpness of the Fresnel transition.
    let fresnel_power = 4.0; 
    // fresnel_effect is typically 0 when view is head-on, 1 at grazing angles.
    let fresnel_effect = pow(1.0 - NdotV, fresnel_power);

    // --- Color and Alpha Calculation ---
    var final_rgb = material_properties.rgb;

    // Mix base color with border highlight
    final_rgb = mix(final_rgb, border_highlight_color, border_factor * border_highlight_strength);

    // Mix with a reflection color based on Fresnel effect (subtle white/light blue reflection)
    let reflection_color = vec3<f32>(0.85, 0.9, 1.0); 
    let reflection_strength = 0.3; // How much reflection color to mix in at grazing angles
    final_rgb = mix(final_rgb, reflection_color, fresnel_effect * reflection_strength);
    
    // Calculate final alpha
    // Start with material's base alpha (for transparency when viewed head-on)
    var final_alpha = material_properties.a;
    // Fresnel effect makes edges appear more opaque/reflective
    // Mix towards a higher alpha value at grazing angles.
    // e.g., if base alpha is 0.15, edges could go up to 0.15 + (1-0.15)*0.7 = 0.15 + 0.595 = 0.745
    let fresnel_opacity_boost = 0.7; 
    final_alpha = mix(final_alpha, final_alpha + (1.0 - final_alpha) * fresnel_opacity_boost, fresnel_effect);
    // Clamp alpha to ensure it stays within [0,1] range, especially if base alpha is high.
    final_alpha = clamp(final_alpha, 0.0, 1.0);

    return vec4<f32>(final_rgb, final_alpha);
}

// Import necessary Bevy PBR bindings and functions
#import bevy_pbr::mesh_functions
#import bevy_pbr::mesh_bindings
#import bevy_pbr::view_bindings

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
    let world_position_vec4 = mesh_functions::mesh_vertex_position_local_to_world(mesh_bindings::mesh.model, vertex_input.position);
    // Transform to clip space
    out.clip_position = view_bindings::view.view_proj * world_position_vec4;
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

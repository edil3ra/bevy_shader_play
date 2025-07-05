#import bevy_pbr::{
    mesh_view_bindings::view,
    forward_io::VertexOutput,
}

struct FresnelMaterial {
    color: vec4<f32>,
};



@group(1) @binding(0) var<uniform> material: FresnelMaterial;




fn saturate(v: f32) -> f32 {
    return clamp(v, 0.0, 1.0);
}

fn F_Schlick_vec(f0: vec3<f32>, f90: f32, u: f32) -> vec3<f32> {
    return f0 + (vec3<f32>(f90) - f0) * pow(1.0 - u, 5.0);
}

fn fresnel(f0: vec3<f32>, LdotH: f32) -> vec3<f32> {
    // f_90 suitable for ambient occlusion
    // see https://google.github.io/filament/Filament.html#lighting/occlusion
    // cheap luminance approximation
    let f90 = saturate(50.0 * f0.g);
    return F_Schlick_vec(f0, f90, LdotH);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let N = normalize(in.world_normal);
    //let V = normalize(view.inverse_view[2].xyz);
    let V = -normalize(vec3<f32>(view.view_from_world[0].z, view.view_from_world[1].z, view.view_from_world[2].z));
    let NdotV = max(dot(N, V), 0.0001);

    let fresnel_color = fresnel(material.color.rgb, NdotV);

    return vec4<f32>(fresnel_color, material.color.a);
}

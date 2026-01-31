#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_bindings::mesh

struct GridParams {
    minor_spacing: f32,
    minor_thickness_px: f32,
    axis_thickness_px: f32,
    fade_distance: f32,
    _pad: vec2<f32>,
    _pad2: vec2<f32>,
};

struct MaterialBindings {
    material: u32,
};

#ifdef BINDLESS
@group(#{MATERIAL_BIND_GROUP}) @binding(0)  var<storage> materials: array<MaterialBindings>;
@group(#{MATERIAL_BIND_GROUP}) @binding(10) var<storage> params_array: binding_array<GridParams>;
#else
@group(#{MATERIAL_BIND_GROUP}) @binding(0)  var<uniform> params: GridParams;
#endif

fn get_params(in: VertexOutput) -> GridParams {
#ifdef BINDLESS
    let slot = mesh[in.instance_index].material_and_lightmap_bind_group_slot & 0xffffu;
    return params_array[materials[slot].material];
#else
    return params;
#endif
}

// Pixel-constant grid line thickness.
// thickness_px is in *screen pixels*, not world units.
fn grid_line_px(coord: f32, spacing: f32, thickness_px: f32) -> f32 {
    let u = coord / spacing;

    // distance to the nearest integer grid line (0 at the line)
    let f = fract(u);
    let d = min(f, 1.0 - f);

    // u-space change across ~1 pixel
    let du = fwidth(u);

    // convert desired pixel thickness to u-space thickness
    let t = thickness_px * du;

    // anti-aliased line
    return 1.0 - smoothstep(t, t + du, d);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let p = in.world_position.xz;
    let params = get_params(in);

    // Minor grid
    let minor = max(
        grid_line_px(p.x, params.minor_spacing, params.minor_thickness_px),
        grid_line_px(p.y, params.minor_spacing, params.minor_thickness_px)
    );

    // Axis lines: X axis is Z=0, Z axis is X=0
    // Pixel-constant axis thickness:
    let dx = fwidth(p.x); // world units per pixel approx
    let dz = fwidth(p.y);

    let z_axis = 1.0 - smoothstep(
        params.axis_thickness_px * dx,
        params.axis_thickness_px * dx + dx,
        abs(p.x)
    );

    let x_axis = 1.0 - smoothstep(
        params.axis_thickness_px * dz,
        params.axis_thickness_px * dz + dz,
        abs(p.y)
    );

    // Fade based on distance from origin (Blender-like “origin grid focus”)
    let r = length(p);
    let fade = 1.0 - smoothstep(params.fade_distance * 0.7, params.fade_distance, r);

    // Colors
    let minor_col = vec3<f32>(0.75, 0.75, 0.75);
    let x_col     = vec3<f32>(0.95, 0.25, 0.25); // X axis (red)
    let z_col     = vec3<f32>(0.25, 0.45, 0.95); // Z axis (blue)

    // Weights (tweak to taste)
    let w_minor = minor * 0.22;
    let w_x     = x_axis * 0.90;
    let w_z     = z_axis * 0.90;

    let alpha = clamp((w_minor + w_x + w_z) * fade, 0.0, 1.0);

    var rgb = vec3<f32>(0.0);
    rgb += minor_col * w_minor;
    rgb += x_col * w_x;
    rgb += z_col * w_z;

    let denom = clamp(w_minor + w_x + w_z, 0.0001, 10.0);
    rgb /= denom;

    return vec4<f32>(rgb, alpha);
}

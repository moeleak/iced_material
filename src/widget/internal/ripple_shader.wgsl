struct RippleUniforms {
    physical: vec4<f32>,
    resolution: vec4<f32>,
    touch_origin: vec4<f32>,
    progress_radius: vec4<f32>,
    color: vec4<f32>,
    sparkle_color: vec4<f32>,
    circle12: vec4<f32>,
    circle3_rotation1: vec4<f32>,
    rotation23: vec4<f32>,
    clip_radius: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: RippleUniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0),
    );

    var output: VertexOutput;
    output.position = vec4<f32>(positions[vertex_index], 0.0, 1.0);
    return output;
}

const PI: f32 = 3.1415926535897932384626;

fn saturate(value: f32) -> f32 {
    return clamp(value, 0.0, 1.0);
}

fn mod2(value: vec2<f32>, divisor: vec2<f32>) -> vec2<f32> {
    return value - divisor * floor(value / divisor);
}

fn threshold(value: f32, low: f32, high: f32) -> f32 {
    if value >= low && value < high {
        return 1.0;
    }

    return 0.0;
}

fn triangle_noise(n: vec2<f32>) -> f32 {
    var point = fract(n * vec2<f32>(5.3987, 5.4421));
    let offset = dot(point.yx, point.xy + vec2<f32>(21.5351, 14.3137));
    point = point + vec2<f32>(offset);

    let xy = point.x * point.y;
    return fract(xy * 95.4307) + fract(xy * 75.04961) - 1.0;
}

fn sparkles(uv: vec2<f32>, time: f32, sparkle_alpha: f32) -> f32 {
    let noise = triangle_noise(uv);
    var sparkle = 0.0;

    for (var index = 0u; index < 4u; index = index + 1u) {
        let i = f32(index);
        let low = i * 0.1;
        let high = low + 0.05;
        let offset = sin(PI * (time + 0.35 * i));

        sparkle = sparkle + threshold(noise + offset, low, high);
    }

    return saturate(sparkle) * sparkle_alpha;
}

fn sub_progress(start: f32, end: f32, progress: f32) -> f32 {
    let sub = clamp(progress, start, end);
    return (sub - start) / (end - start);
}

fn soft_circle(point: vec2<f32>, center: vec2<f32>, radius: f32, blur: f32) -> f32 {
    if radius <= 0.0 {
        return 0.0;
    }

    let blur_half = blur * 0.5;
    let dist = distance(point, center);

    return 1.0 - smoothstep(1.0 - blur_half, 1.0 + blur_half, dist / radius);
}

fn soft_ring(point: vec2<f32>, center: vec2<f32>, radius: f32, progress: f32, blur: f32) -> f32 {
    let thickness = 0.05 * radius;
    let current_radius = radius * progress;
    let outer = soft_circle(point, center, current_radius + thickness, blur);
    let inner = soft_circle(point, center, max(current_radius - thickness, 0.0), blur);

    return saturate(outer - inner);
}

fn rotate2d(rotation: vec2<f32>, delta: vec2<f32>) -> vec2<f32> {
    let matrix = mat2x2<f32>(
        vec2<f32>(rotation.x, -rotation.y),
        vec2<f32>(rotation.y, rotation.x),
    );

    return matrix * delta;
}

fn circle_grid(
    resolution: vec2<f32>,
    coord: vec2<f32>,
    center: vec2<f32>,
    rotation: vec2<f32>,
    cell_diameter: f32,
) -> f32 {
    let rotated = rotate2d(rotation, center - coord) + center;
    let cell = mod2(rotated, vec2<f32>(cell_diameter)) / resolution;
    let normal_radius = cell_diameter / resolution.y * 0.5;
    let radius = 0.65 * normal_radius;

    return soft_circle(
        cell,
        vec2<f32>(normal_radius),
        radius,
        radius * 50.0,
    );
}

fn turbulence(uv: vec2<f32>) -> f32 {
    let scale = vec2<f32>(0.8);
    let scaled_uv = uv * scale;
    let circle1 = uniforms.circle12.xy;
    let circle2 = uniforms.circle12.zw;
    let circle3 = uniforms.circle3_rotation1.xy;
    let rotation1 = uniforms.circle3_rotation1.zw;
    let rotation2 = uniforms.rotation23.xy;
    let rotation3 = uniforms.rotation23.zw;
    let grid1 = circle_grid(scale, scaled_uv, circle1, rotation1, 0.17);
    let grid2 = circle_grid(scale, scaled_uv, circle2, rotation2, 0.2);
    let grid3 = circle_grid(scale, scaled_uv, circle3, rotation3, 0.275);
    let value = (grid1 * grid1 + grid2 - grid3) * 0.5;

    return saturate(0.45 + 0.8 * value);
}

fn corner_mask(point: vec2<f32>, center: vec2<f32>, radius: f32) -> f32 {
    if radius <= 0.0 || distance(point, center) <= radius {
        return 1.0;
    }

    return 0.0;
}

fn rounded_rect_mask(point: vec2<f32>, size: vec2<f32>) -> f32 {
    if uniforms.physical.w < 0.5 {
        return 1.0;
    }

    if point.x < 0.0 || point.y < 0.0 || point.x > size.x || point.y > size.y {
        return 0.0;
    }

    let max_radius = min(size.x, size.y) * 0.5;
    let top_left = clamp(uniforms.clip_radius.x, 0.0, max_radius);
    let top_right = clamp(uniforms.clip_radius.y, 0.0, max_radius);
    let bottom_right = clamp(uniforms.clip_radius.z, 0.0, max_radius);
    let bottom_left = clamp(uniforms.clip_radius.w, 0.0, max_radius);

    if point.x < top_left && point.y < top_left {
        return corner_mask(point, vec2<f32>(top_left, top_left), top_left);
    }

    if point.x > size.x - top_right && point.y < top_right {
        return corner_mask(point, vec2<f32>(size.x - top_right, top_right), top_right);
    }

    if point.x > size.x - bottom_right && point.y > size.y - bottom_right {
        return corner_mask(
            point,
            vec2<f32>(size.x - bottom_right, size.y - bottom_right),
            bottom_right,
        );
    }

    if point.x < bottom_left && point.y > size.y - bottom_left {
        return corner_mask(point, vec2<f32>(bottom_left, size.y - bottom_left), bottom_left);
    }

    return 1.0;
}

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let scale_factor = uniforms.physical.z;
    let point = (position.xy - uniforms.physical.xy) / scale_factor;
    let size = uniforms.resolution.xy;
    let progress = uniforms.progress_radius.x;
    let max_radius = uniforms.progress_radius.y;
    let noise_phase = uniforms.progress_radius.z;
    let fade_in = sub_progress(0.0, 0.13, progress);
    let scale_in = sub_progress(0.0, 1.0, progress);
    let fade_out_noise = sub_progress(0.4, 0.5, progress);
    let fade_out_ripple = sub_progress(0.4, 1.0, progress);
    let touch = uniforms.touch_origin.xy;
    let origin = uniforms.touch_origin.zw;
    let center = mix(touch, origin, saturate(progress * 2.0));
    let ring = soft_ring(point, center, max_radius, scale_in, 1.0);
    let alpha = min(fade_in, 1.0 - fade_out_noise);
    let uv = point / size;
    let density_uv = uv - mod2(uv, uniforms.resolution.zw);
    let turb = turbulence(uv);
    let sparkle_alpha =
        sparkles(density_uv, noise_phase, uniforms.sparkle_color.a) * ring * alpha * turb;
    let fade = min(fade_in, 1.0 - fade_out_ripple);
    let wave_alpha =
        soft_circle(point, center, max_radius * scale_in, 1.0) * fade * uniforms.color.a;
    let wave_color = vec4<f32>(uniforms.color.rgb * wave_alpha, wave_alpha);
    let sparkle_color = vec4<f32>(
        uniforms.sparkle_color.rgb * uniforms.sparkle_color.a,
        uniforms.sparkle_color.a,
    );
    let mask = rounded_rect_mask(point, size);

    return mix(wave_color, sparkle_color, sparkle_alpha) * mask;
}

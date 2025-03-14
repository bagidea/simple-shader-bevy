@group(2) @binding(0) var<uniform> iTime: f32;
@group(2) @binding(1) var<uniform> iResolution: vec3<f32>;

fn palette(t: f32, a: vec3<f32>, b: vec3<f32>, c: vec3<f32>, d: vec3<f32>) -> vec3<f32> {
    return a + b * cos(6.28318 * (c * t + d));
}

@fragment
fn fragment(@builtin(position) fragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    var uv: vec2<f32> = ((fragCoord.xy * 2.0 - iResolution.xy) / iResolution.y) * -1.0;
	
    let uv0: vec2<f32> = uv;
    let d0: f32 = length(uv0);

    var final_color: vec3<f32> = vec3<f32>(0.0);

    for (var i: f32 = 0.0; i < 4.0; i = i + 1.0) {
        uv = fract(uv * 1.5) - 0.5;
        var d: f32 = length(uv) * exp(-d0);

        var color: vec3<f32> = palette(
            d0 + i * 0.4 + iTime,
            vec3<f32>(0.2, 0.7, 0.4),
            vec3<f32>(0.6, 0.9, 0.2),
            vec3<f32>(0.6, 0.8, 0.7),
            vec3<f32>(0.5, 0.1, 0.0)
        );

        d = sin(d * 8.0 + iTime * 0.4) / 8.0;
        d = abs(d);
        d = pow(0.01 / d, 1.2);

        color = color * d;
        final_color = final_color + color;
    }

    return vec4<f32>(final_color, 1.0);
}
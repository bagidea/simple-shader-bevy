#version 450

layout(set = 2, binding = 0) uniform float iTime;
layout(set = 2, binding = 1) uniform vec3 iResolution;

layout(location = 0) out vec4 fragColor;

// Gradient color procedural - cosine wave - glsl cos palette
vec3 palette(float t, vec3 a, vec3 b, vec3 c, vec3 d){
    return a + b * cos(6.28318 * (c * t + d));
}

void mainImage(out vec4 fragColor, in vec2 fragCoord)
{
    vec2 uv = (fragCoord * 2. - iResolution.xy) / iResolution.y;
    uv.y *= -1.; // Flip Y to follow Shadertoy

    vec2 uv0 = uv;
  	float d0 = length(uv0);

  	vec3 final_color = vec3(0.0);

  	for (float i = 0.0; i < 4.0; i++) {
      //uv = uv - floor(uv); // fract(uv);
    	uv = fract(uv * 1.5) - 0.5;

      //float d = sqrt(pow(uv.x, 2.0) + pow(uv.y, 2.0)); // length(uv)
      float d = length(uv) * exp(-d0);
      //float d = uv.x;

      //vec3 color = vec3(0.3, 0.6, 1.0);
      //vec3 color = vec3(1.0, 2.0, 3.0);
      vec3 color = palette(
          d0 + i * 0.4 + iTime,
          vec3(0.2,0.7,0.4),
          vec3(0.6,0.9,0.2),
          vec3(0.6,0.8,0.7),
          vec3(0.5,0.1,0.0)
      );

      //d -= 0.5;
      d = sin(d * 8.0 + iTime * 0.4) / 8.0;
      d  = abs(d);

      //d = step(0.1, d);
      //d = smoothstep(0.0, 0.1, d);
      d = pow(0.01 / d, 1.2);

      color *= d; 
      final_color += color;
    }

    fragColor = vec4(final_color, 1.0);
}

void main() {
    mainImage(fragColor, gl_FragCoord.xy);
}
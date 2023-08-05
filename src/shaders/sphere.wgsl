struct Uniforms {
  radius : f32,
  size : f32,
  matrix: mat4x4<f32>,
};

@group(0) @binding(0) var<uniform> uniforms : Uniforms;

fn spherical_to_cartesian(u : vec2f) -> vec3f {
    let v = 2.0 * 3.1415926538 * u;
    let x = cos(v.x) * sin(v.y);
    let y = sin(v.x);
    let z = cos(v.x) * cos(v.y);
    return vec3f(x, y, z);
}

fn get_uv(i : u32) -> vec2f {
    let four = u32(4);
    let size = u32(uniforms.size);
    let step = 1.0 / f32(size);
    let x = i % four;
    var u = f32(i / four / size) * step;
    var v = f32(i / four % size) * step;
    if (i % four == u32(2)){
      u = u + step;
    }
    if (i % four == u32(1)){
      v = v + step;
    }
    return vec2f(u, v);
}

@vertex
fn vert_main(@builtin(vertex_index) i : u32) -> @builtin(position) vec4f {
    let uv = get_uv(i);
    let center = vec3f(0.0, 0.0, 0.0);
    let pos = spherical_to_cartesian(uv) * uniforms.radius + center;
    return uniforms.matrix * vec4f(pos, 1.0);
}

@fragment
fn frag_main(@builtin(position) coord: vec4f) -> @location(0) vec4f {
  let color = vec4f(1.0, 1.0, 1.0, 1.0);
  return color;
}
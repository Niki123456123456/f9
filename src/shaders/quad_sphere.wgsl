struct Uniforms {
  radius : f32,
  size : f32,
  matrix: mat4x4<f32>,
};

@group(0) @binding(0) var<uniform> uniforms : Uniforms;


fn get_uv(i : u32) -> vec2f {
    let four = u32(4);
    let size = u32(uniforms.size);
    let step = 1.0 / f32(size);
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
     var poss = array<vec3f, 6>(
      vec3f(1.0,  0.0, 0.0),
      vec3f(-1.0,  0.0, 0.0),
      vec3f(0.0,  1.0, 0.0),
      vec3f(0.0,  -1.0, 0.0),
      vec3f(0.0,  0.0, 1.0),
      vec3f(0.0,  0.0, -1.0));

    var pos = poss[i / u32(256)]; 
    var a = vec3f(pos.y, pos.z, pos.x);
    var b = vec3f(pos.z, pos.x, pos.y);
    
    let uv = get_uv(i % u32(256)); 
    let p = pos + (uv.x - 0.5) * 2.0 * a - (uv.y - 0.5) * 2.0 * b;
    let pn = normalize(p);
    return uniforms.matrix * vec4f(pn, 1.0);
    //return uniforms.matrix * vec4f(uv, 1.0, 1.0);
}

@fragment
fn frag_main(@builtin(position) coord: vec4f) -> @location(0) vec4f {
  let color = vec4f(1.0, 1.0, 1.0, 1.0);
  return color;
}
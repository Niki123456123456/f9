@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(1) @binding(0) var<storage, read> vertexBuffer : VertexBuffer;


@vertex
fn vert_main(@builtin(vertex_index) i : u32) -> @builtin(position) vec4f {
  let axis = vertexBuffer.values[i / u32(4)];
  let p = vec3f(axis.px, axis.py, axis.pz);
  let d = vec3f(axis.dx, axis.dy, axis.dz);
  let l = 100000.0;
  let pos = p + f32(i % u32(2)) * d * l - f32((i / u32(2)) % u32(2)) * d * l;
  return uniforms.matrix * vec4f(pos, 1.0);
}

@fragment
fn frag_main(@builtin(position) coord: vec4f) -> @location(0) vec4f {
  let color = vec4f(1.0, 1.0, 1.0, 1.0);
  return color;
}
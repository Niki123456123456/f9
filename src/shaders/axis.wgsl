struct Uniforms {
  width : f32,
  height : f32,
  camera_orientation_x : f32,
  camera_orientation_y : f32,
  camera_orientation_z : f32,
  matrix: mat4x4<f32>,
};
struct Vertex {
  px : f32,
  py : f32,
  pz : f32,
  dx : f32,
  dy : f32,
  dz : f32,
  flags : i32,
}
struct VertexBuffer {
  values: array<Vertex>,
};

@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(0) @binding(1) var<storage, read> vertexBuffer : VertexBuffer;


@vertex
fn vert_main(@builtin(vertex_index) i : u32) -> @builtin(position) vec4f {
  let axis = vertexBuffer.values[i / u32(6)];
  let p = vec3f(axis.px, axis.py, axis.pz);
  let d = vec3f(axis.dx, axis.dy, axis.dz);
  return uniforms.matrix * vec4f(p, 1.0);
}

@fragment
fn frag_main(@builtin(position) coord: vec4f) -> @location(0) vec4f {
  let color = vec4f(1.0, 1.0, 1.0, 1.0);
  return color;
}
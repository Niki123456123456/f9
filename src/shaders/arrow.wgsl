struct Uniforms {
  width : f32,
  height : f32,
  height_top : f32,
  camera_orientation_x : f32,
  camera_orientation_y : f32,
  camera_orientation_z : f32,
  mouse_x : f32,
  mouse_y : f32,
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
@group(0) @binding(3) var<storage, read> vertexBuffer : VertexBuffer;


@vertex
fn vert_main(@builtin(vertex_index) i : u32) -> @builtin(position) vec4f {
  let axis = vertexBuffer.values[i / u32(6)];
  let a = vec3f(axis.px, axis.py, axis.pz);
  let arrow_direction = vec3f(axis.dx, axis.dy, axis.dz);
  let spacing = 0.1;
  let camera_orientation = vec3f(uniforms.camera_orientation_x, uniforms.camera_orientation_y, uniforms.camera_orientation_z);

  let p1 = a + arrow_direction * (1. - spacing);
  let p2 = a + arrow_direction * spacing;
  let p0 = a + arrow_direction * (1. - 2. * spacing);
  let p3 = p0 + cross(arrow_direction, camera_orientation) * spacing;
  let p4 = p0 - cross(arrow_direction, camera_orientation) * spacing;

  var pos = array<vec3f, 6>();
  pos[0] = p1;
  pos[1] = p2;
  pos[2] = p1;
  pos[3] = p3;
  pos[4] = p1;
  pos[5] = p4;

  return uniforms.matrix * vec4f(pos[i % u32(6)], 1.0);
}

@fragment
fn frag_main(@builtin(position) coord: vec4f) -> @location(0) vec4f {
  let color = vec4f(1.0, 1.0, 1.0, 1.0);
  return color;
}

struct Uniforms {
  width : f32,
  height : f32,
  height_top : f32,
  camera_orientation_x : f32,
  camera_orientation_y : f32,
  camera_orientation_z : f32,
  mouse_x : f32,
  mouse_y : f32,
  camera_origin_x : f32,
  camera_origin_y : f32,
  camera_origin_z : f32,
  camera_orient_x : f32,
  camera_orient_y : f32,
  camera_orient_z : f32,
  matrix: mat4x4<f32>,
};
struct Point {
  px : f32,
  py : f32,
  pz : f32,
  flags : i32,
}
struct PointBuffer {
  values: array<Point>,
};
struct Line {
  point_a : u32,
  point_b : u32,
  flags : i32,
}
struct LineBuffer {
  values: array<Line>,
};
struct VertexOutput {
  @builtin(position) position : vec4f,
  @location(0) flags : i32,
};

@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(0) @binding(4) var<storage, read> pointBuffer : PointBuffer;
@group(0) @binding(5) var<storage, read> lineBuffer : LineBuffer;


@vertex
fn vert_main(@builtin(vertex_index) i : u32) -> VertexOutput {
  let line = lineBuffer.values[i / u32(2)];
  var pos = array<vec3f, 2>();
  pos[0] = vec3f(pointBuffer.values[line.point_a].px, pointBuffer.values[line.point_a].py, pointBuffer.values[line.point_a].pz);
  pos[1] = vec3f(pointBuffer.values[line.point_b].px, pointBuffer.values[line.point_b].py, pointBuffer.values[line.point_b].pz);

  var output : VertexOutput;
  output.position = uniforms.matrix * vec4f(pos[i % u32(2)], 1.0);
  output.flags = line.flags;
  return output;
}

@fragment
fn frag_main(v: VertexOutput) -> @location(0) vec4f {
  var color = vec4f(1.0, 1.0, 1.0, 1.0);
  if ((v.flags & 2) == 2){ // hover
    color =  vec4f(1.0, 0.0, 0.0, 1.0);
  }
  return color;
}
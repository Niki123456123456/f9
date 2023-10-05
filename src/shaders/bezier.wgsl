struct Uniforms {
  width : f32,
  height : f32,
  camera_orientation_x : f32,
  camera_orientation_y : f32,
  camera_orientation_z : f32,
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
struct Bezier {
  point_a : u32,
  point_b : u32,
  control_a : u32,
  control_b : u32,
  flags : i32,
}
struct BezierBuffer {
  values: array<Bezier>,
};

@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(0) @binding(4) var<storage, read> pointBuffer : PointBuffer;
@group(0) @binding(6) var<storage, read> bezierBuffer : BezierBuffer;

fn get_position(t : f32, point_a : vec3f, point_b : vec3f, control_a : vec3f, control_b : vec3f) -> vec3f{
    let position = pow(1.0 - t, 3.0) * point_a
        + 3.0 * pow(1.0 - t, 2.0) * t * control_a
        + 3.0 * (1.0 - t) * pow(t, 2.0) * control_b
        + pow(t, 3.0) * point_b;
    return position;
}

@vertex
fn vert_main(@builtin(vertex_index) i : u32) -> @builtin(position) vec4f {
  let bezier = bezierBuffer.values[i / u32(51)];
  let point_a = vec3f(pointBuffer.values[bezier.point_a].px, pointBuffer.values[bezier.point_a].py, pointBuffer.values[bezier.point_a].pz);
  let point_b = vec3f(pointBuffer.values[bezier.point_b].px, pointBuffer.values[bezier.point_b].py, pointBuffer.values[bezier.point_b].pz);
  let control_a = vec3f(pointBuffer.values[bezier.control_a].px, pointBuffer.values[bezier.control_a].py, pointBuffer.values[bezier.control_a].pz);
  let control_b = vec3f(pointBuffer.values[bezier.control_b].px, pointBuffer.values[bezier.control_b].py, pointBuffer.values[bezier.control_b].pz);

  let t = f32(i % u32(51)) / 50.0;

  return uniforms.matrix * vec4f(get_position(t, point_a, point_b, control_a, control_b), 1.0);
}

@fragment
fn frag_main(@builtin(position) coord: vec4f) -> @location(0) vec4f {
  let color = vec4f(1.0, 1.0, 1.0, 1.0);
  return color;
}
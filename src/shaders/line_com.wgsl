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

@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(0) @binding(4) var<storage, read> pointBuffer : PointBuffer;
@group(0) @binding(5) var<storage, read_write> lineBuffer : LineBuffer;

fn perp(a : vec2f, b : vec2f, c : vec2f) -> f32 { // perpendicular distance between line a+t*b and point c
    let distance = length(c - a - dot(c - a, b) * b);
    return distance;
}
fn perp_t(a : vec2f, b : vec2f, c : vec2f) -> f32{
    let t = min(1.0, max(0.0, dot(c - a, b) / dot(b, b)));
    return t;
}

fn to_screen_position(position : vec3f) -> vec2f {
    let pos =  uniforms.matrix * vec4f(position, 1.0);
    return vec4f(((pos.xyz/pos.w) * 0.5 + 0.5) * vec3(uniforms.width, uniforms.height, 1.0), pos.w).xy;
}

@compute @workgroup_size(1, 1)
fn main(@builtin(global_invocation_id) i : vec3<u32>) {
  let line = lineBuffer.values[i.x];

  let point_a = vec3f(pointBuffer.values[line.point_a].px, pointBuffer.values[line.point_a].py, pointBuffer.values[line.point_a].pz);
  let point_b = vec3f(pointBuffer.values[line.point_b].px, pointBuffer.values[line.point_b].py, pointBuffer.values[line.point_b].pz);

  let pos_a = to_screen_position(point_a);
  let pos_b = to_screen_position(point_b);

  let mouse_pos = vec2(uniforms.mouse_x, uniforms.mouse_y);

  var d = perp(pos_a, normalize(pos_b - pos_a), mouse_pos);
  let t = perp_t(pos_a, pos_b - pos_a, mouse_pos);
  if (t <= 0.0){
    d = distance(pos_a, mouse_pos);
  } else if (t >= 1.0) {
    d = distance(pos_b, mouse_pos);
  }

  let point = point_a + t * (point_b - point_a);

  if(d <= 20.){
    lineBuffer.values[i.x].flags = lineBuffer.values[i.x].flags | 2;
  } else {
    lineBuffer.values[i.x].flags = lineBuffer.values[i.x].flags & (~2);
  }

}
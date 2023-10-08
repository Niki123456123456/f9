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
@group(0) @binding(4) var<storage, read> pointBuffer : PointBuffer;
@group(0) @binding(3) var<storage, read_write> vertexBuffer : VertexBuffer;

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

fn get_distance(pos_a : vec2f, pos_b : vec2f, mouse_pos : vec2f, t : f32) -> f32 {
  if (t <= 0.0){
    return distance(pos_a, mouse_pos);
  } else if (t >= 1.0) {
    return distance(pos_b, mouse_pos);
  }
  return perp(pos_a, normalize(pos_b - pos_a), mouse_pos);
}

@compute @workgroup_size(1, 1)
fn main(@builtin(global_invocation_id) i : vec3<u32>) {
  let arrow = vertexBuffer.values[i.x];
  let a = vec3f(arrow.px, arrow.py, arrow.pz);
  let arrow_direction = vec3f(arrow.dx, arrow.dy, arrow.dz);
  let spacing = 0.1;
  let camera_orientation = vec3f(uniforms.camera_orientation_x, uniforms.camera_orientation_y, uniforms.camera_orientation_z);

  let point_a = a + arrow_direction * (1. - spacing);
  let point_b = a + arrow_direction * spacing;

  let pos_a = to_screen_position(point_a);
  let pos_b = to_screen_position(point_b);

  let mouse_pos = vec2(uniforms.mouse_x, uniforms.mouse_y);

  let t = perp_t(pos_a, pos_b - pos_a, mouse_pos);
  let d = get_distance(pos_a, pos_b, mouse_pos, t);

  let point = point_a + t * (point_b - point_a);

  if(d <= 20.){
    vertexBuffer.values[i.x].flags = vertexBuffer.values[i.x].flags | 2;
  } else {
    vertexBuffer.values[i.x].flags = vertexBuffer.values[i.x].flags & (~2);
  }

}
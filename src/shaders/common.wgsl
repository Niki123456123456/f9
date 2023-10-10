const PI: f32 = 3.1415926538;

// structs:
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
struct Vertex {
  px : f32,
  py : f32,
  pz : f32,
  dx : f32,
  dy : f32,
  dz : f32,
  flags : i32,
}
struct Point {
  px : f32,
  py : f32,
  pz : f32,
  flags : i32,
}
struct Line {
  point_a : u32,
  point_b : u32,
  flags : i32,
}
struct Bezier {
  point_a : u32,
  point_b : u32,
  control_a : u32,
  control_b : u32,
  flags : i32,
}
struct Circle {
  center: u32,
  radius: f32,
  orientation_x : f32,
  orientation_y : f32,
  orientation_z : f32,
  heightfactor: f32,
  flags : i32,
}

// buffers
struct LineBuffer {
  values: array<Line>,
};
struct VertexBuffer {
  values: array<Vertex>,
};
struct PointBuffer {
  values: array<Point>,
};
struct BezierBuffer {
  values: array<Bezier>,
};
struct CircleBuffer {
  values: array<Circle>,
};

fn to_screen_position(position : vec3f) -> vec2f {
    let pos =  uniforms.matrix * vec4f(position, 1.0);
    return vec4f(((pos.xyz/pos.w) * 0.5 + 0.5) * vec3(uniforms.width, uniforms.height, 1.0), pos.w).xy;
}

fn shift(v : vec3f) -> vec3f{
    return vec3f(v.z, v.x, v.y);
}
fn double_shift(v : vec3f)-> vec3f{
    return vec3f(v.y, v.z, v.x);
}


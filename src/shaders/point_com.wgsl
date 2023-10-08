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

@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(0) @binding(4) var<storage, read_write> pointBuffer : PointBuffer;

fn to_screen_position(position : vec3f) -> vec2f {
    let pos =  uniforms.matrix * vec4f(position, 1.0);
    return vec4f(((pos.xyz/pos.w) * 0.5 + 0.5) * vec3(uniforms.width, uniforms.height, 1.0), pos.w).xy;
}

@compute @workgroup_size(1, 1)
fn main(@builtin(global_invocation_id) i : vec3<u32>) {
  let point = pointBuffer.values[i.x];

  let pos = vec3f(point.px, point.py, point.pz);
  let position = to_screen_position(pos);
  let d = distance(position, vec2(uniforms.mouse_x, uniforms.height - uniforms.mouse_y));

  if(d <= 20.){
    pointBuffer.values[i.x].flags = pointBuffer.values[i.x].flags | 2;
  } else {
    pointBuffer.values[i.x].flags = pointBuffer.values[i.x].flags & (~2);
  }

}
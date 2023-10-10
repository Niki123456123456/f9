@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(0) @binding(4) var<storage, read_write> pointBuffer : PointBuffer;
@group(0) @binding(5) var<storage, read_write> lineBuffer : LineBuffer;

fn perp(a : vec2f, b : vec2f, c : vec2f) -> f32 { // perpendicular distance between line a+t*b and point c
    let distance = length(c - a - dot(c - a, b) * b);
    return distance;
}
fn perp_t(a : vec2f, b : vec2f, c : vec2f) -> f32{
    let t = min(1.0, max(0.0, dot(c - a, b) / dot(b, b)));
    return t;
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
  let line = lineBuffer.values[i.x];

  let point_a = vec3f(pointBuffer.values[line.point_a].px, pointBuffer.values[line.point_a].py, pointBuffer.values[line.point_a].pz);
  let point_b = vec3f(pointBuffer.values[line.point_b].px, pointBuffer.values[line.point_b].py, pointBuffer.values[line.point_b].pz);

  let pos_a = to_screen_position(point_a);
  let pos_b = to_screen_position(point_b);

  let mouse_pos = vec2(uniforms.mouse_x, uniforms.mouse_y);

  let t = perp_t(pos_a, pos_b - pos_a, mouse_pos);
  let d = get_distance(pos_a, pos_b, mouse_pos, t);

  let point = point_a + t * (point_b - point_a);

  if(d <= 20.){
    lineBuffer.values[i.x].flags = lineBuffer.values[i.x].flags | 2;
  } else {
    lineBuffer.values[i.x].flags = lineBuffer.values[i.x].flags & (~2);
  }

}
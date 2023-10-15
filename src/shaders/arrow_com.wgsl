@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(1) @binding(0) var<storage, read_write> hoverCounter : AtomicCounter;
@group(1) @binding(1) var<storage, read_write> hoverBuffer : HoverBuffer;
@group(2) @binding(0) var<storage, read_write> vertexBuffer : VertexBuffer;

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
    let hover_index = atomicAdd(&hoverCounter.counter, 1u);
    hoverBuffer.values[hover_index].index = i.x;
    hoverBuffer.values[hover_index].ctype = 5; // arrow
    hoverBuffer.values[hover_index].distance = distance(point, vec3f(uniforms.camera_origin_x, uniforms.camera_origin_y, uniforms.camera_origin_z));
    hoverBuffer.values[hover_index].position_x = point.x;
    hoverBuffer.values[hover_index].position_y = point.y;
    hoverBuffer.values[hover_index].position_z = point.z;
  } else {
    vertexBuffer.values[i.x].flags = vertexBuffer.values[i.x].flags & (~2);
  }

}
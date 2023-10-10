@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(0) @binding(4) var<storage, read_write> pointBuffer : PointBuffer;

@compute @workgroup_size(1, 1)
fn main(@builtin(global_invocation_id) i : vec3<u32>) {
  let point = pointBuffer.values[i.x];

  let pos = vec3f(point.px, point.py, point.pz);
  let position = to_screen_position(pos);
  let d = distance(position, vec2(uniforms.mouse_x, uniforms.mouse_y));

  if(d <= 20.){
    pointBuffer.values[i.x].flags = pointBuffer.values[i.x].flags | 2;
  } else {
    pointBuffer.values[i.x].flags = pointBuffer.values[i.x].flags & (~2);
  }

}
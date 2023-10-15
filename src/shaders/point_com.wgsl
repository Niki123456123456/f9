@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(1) @binding(0) var<storage, read_write> hoverCounter : AtomicCounter;
@group(1) @binding(1) var<storage, read_write> hoverBuffer : HoverBuffer;
@group(2) @binding(0) var<storage, read_write> pointBuffer : PointBuffer;


@compute @workgroup_size(1, 1)
fn main(@builtin(global_invocation_id) i : vec3<u32>) {
  let point = pointBuffer.values[i.x];

  let pos = vec3f(point.px, point.py, point.pz);
  let position = to_screen_position(pos);
  let d = distance(position, vec2(uniforms.mouse_x, uniforms.mouse_y));

  if(d <= 20.){
    pointBuffer.values[i.x].flags = pointBuffer.values[i.x].flags | 2;
    let hover_index = atomicAdd(&hoverCounter.counter, 1u);
    hoverBuffer.values[hover_index].index = i.x;
    hoverBuffer.values[hover_index].ctype = 1; // point
    hoverBuffer.values[hover_index].distance = distance(pos, vec3f(uniforms.camera_origin_x, uniforms.camera_origin_y, uniforms.camera_origin_z));
    hoverBuffer.values[hover_index].position_x = pos.x;
    hoverBuffer.values[hover_index].position_y = pos.y;
    hoverBuffer.values[hover_index].position_z = pos.z;
  } else {
    pointBuffer.values[i.x].flags = pointBuffer.values[i.x].flags & (~2);
  }

}
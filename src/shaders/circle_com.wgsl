@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(1) @binding(0) var<storage, read_write> hoverCounter : AtomicCounter;
@group(1) @binding(1) var<storage, read_write> hoverBuffer : HoverBuffer;
@group(2) @binding(0) var<storage, read_write> pointBuffer : PointBuffer;
@group(2) @binding(1) var<storage, read_write> circleBuffer : CircleBuffer;

fn get_circle(t : f32, orientation : vec3f, heightFactor : f32, center : vec3f, radius : f32) -> vec3f {
    let x = (2.0 * PI * t);
    let orient = shift(orientation) * cos(x) + double_shift(orientation) * sin(x) + orientation * heightFactor * t;
    let position = center + radius * orient;
    return position;
}

fn newtonMinDist(c : vec3f, o1 : vec3f, o2: vec3f, o3: vec3f, r : f32, h : f32, m : vec3f, b : vec3f) -> f32 {
    var t1 = 0.0;
    var min_dist: f32 = 1000000.0;
    for (var i : u32 = 0u; i < 360u; i = i + 1u) {
        let angle = f32(i) * 0.0174532925; // Umrechnung von Grad in Radiant
        let t1_temp = angle / (2.0 * PI);
        let k_t1 = c + r * (o1 * cos(2.0 * PI * t1_temp) + o2 * sin(2.0 * PI * t1_temp) + o3 * h * t1_temp);
        let t2_temp = dot(k_t1 - b, m) / dot(m, m);
        let s_t2 = m * t2_temp + b;
        let dist = distance(k_t1, s_t2);

        if (dist < min_dist) {
            min_dist = dist;
            t1 = t1_temp;
        }
    }

return t1;
}

@compute @workgroup_size(1, 1)
fn main(@builtin(global_invocation_id) i : vec3<u32>) {
  let circle = circleBuffer.values[i.x];

  let center = vec3f(pointBuffer.values[circle.center].px, pointBuffer.values[circle.center].py, pointBuffer.values[circle.center].pz);
  let radius = circle.radius;
  let orient = vec3f(circle.orientation_x, circle.orientation_y, circle.orientation_z);
  let heightfactor = circle.heightfactor;

  let camera_dir = vec3f(uniforms.camera_orient_x, uniforms.camera_orient_y, uniforms.camera_orient_z);
  let camera_origin = vec3f(uniforms.camera_origin_x, uniforms.camera_origin_y, uniforms.camera_origin_z);

  let t = newtonMinDist(center, shift(orient), double_shift(orient), orient, radius, heightfactor, camera_dir, camera_origin);
  let pos = get_circle(t, orient, heightfactor, center, radius);
  let position = to_screen_position(pos);
  let d = distance(position, vec2(uniforms.mouse_x, uniforms.mouse_y));

  if(d <= 20.){
    circleBuffer.values[i.x].flags = circleBuffer.values[i.x].flags | 2;
    let hover_index = atomicAdd(&hoverCounter.counter, 1u);
    hoverBuffer.values[hover_index].index = i.x;
    hoverBuffer.values[hover_index].ctype = 3; // circle
    hoverBuffer.values[hover_index].distance = distance(pos, vec3f(uniforms.camera_origin_x, uniforms.camera_origin_y, uniforms.camera_origin_z));
    hoverBuffer.values[hover_index].position_x = pos.x;
    hoverBuffer.values[hover_index].position_y = pos.y;
    hoverBuffer.values[hover_index].position_z = pos.z;
  } else {
    circleBuffer.values[i.x].flags = circleBuffer.values[i.x].flags & (~2);
  }

  

}
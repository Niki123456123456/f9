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
struct Circle {
  center: u32,
  radius: f32,
  orientation_x : f32,
  orientation_y : f32,
  orientation_z : f32,
  heightfactor: f32,
  flags : i32,
}
struct CircleBuffer {
  values: array<Circle>,
};

@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(0) @binding(4) var<storage, read> pointBuffer : PointBuffer;
@group(0) @binding(7) var<storage, read_write> circleBuffer : CircleBuffer;

fn to_screen_position(position : vec3f) -> vec2f {
    let pos =  uniforms.matrix * vec4f(position, 1.0);
    return vec4f(((pos.xyz/pos.w) * 0.5 + 0.5) * vec3(uniforms.width, uniforms.height, 1.0), pos.w).xy;
}

fn shift(v : vec3f) -> vec3f {
    return vec3f(v.z, v.x, v.y);
}
fn double_shift(v : vec3f) -> vec3f {
    return vec3f(v.y, v.z, v.x);
}

fn get_circle(t : f32, orientation : vec3f, heightFactor : f32, center : vec3f, radius : f32) -> vec3f {
    let x = (2.0 * 3.14159265 * t);
    let orient = shift(orientation) * cos(x) + double_shift(orientation) * sin(x) + orientation * heightFactor * t;
    let position = center + radius * orient;
    return position;
}

fn newtonMinDist(c : vec3f, o1 : vec3f, o2: vec3f, o3: vec3f, r : f32, h : f32, m : vec3f, b : vec3f) -> f32 {
    var t1 = 0.0;
    var min_dist: f32 = 1000000.0;
    for (var i : u32 = 0u; i < 360u; i = i + 1u) {
        let angle = f32(i) * 0.0174532925; // Umrechnung von Grad in Radiant
        let t1_temp = angle / (2.0 * 3.14159265);
        let k_t1 = c + r * (o1 * cos(2.0 * 3.14159265 * t1_temp) + o2 * sin(2.0 * 3.14159265 * t1_temp) + o3 * h * t1_temp);
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
  } else {
    circleBuffer.values[i.x].flags = circleBuffer.values[i.x].flags & (~2);
  }

  

}
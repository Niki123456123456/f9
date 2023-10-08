struct Uniforms {
  width : f32,
  height : f32,
  height_top : f32,
  camera_orientation_x : f32,
  camera_orientation_y : f32,
  camera_orientation_z : f32,
  mouse_x : f32,
  mouse_y : f32,
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
struct Bezier {
  point_a : u32,
  point_b : u32,
  control_a : u32,
  control_b : u32,
  flags : i32,
}
struct BezierBuffer {
  values: array<Bezier>,
};

@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(0) @binding(4) var<storage, read> pointBuffer : PointBuffer;
@group(0) @binding(6) var<storage, read_write> bezierBuffer : BezierBuffer;

fn to_screen_position(position : vec3f) -> vec2f {
    let pos =  uniforms.matrix * vec4f(position, 1.0);
    return vec4f(((pos.xyz/pos.w) * 0.5 + 0.5) * vec3(uniforms.width, uniforms.height, 1.0), pos.w).xy;
}

fn get_position(P0: vec3f, P1: vec3f, P2: vec3f, P3: vec3f, t : f32) -> vec3f {
    let position = pow(1.0 - t, 3.0) * P0
        + 3.0 * pow(1.0 - t, 2.0) * t * P1
        + 3.0 * (1.0 - t) * pow(t, 2.0) * P2
        + pow(t, 3.0) * P3;
    return position;
}

fn cubicBezier(P0 : vec2f, P1 : vec2f, P2 : vec2f, P3 : vec2f, t : f32) -> vec2f {
    let u = 1.0 - t;
    let tt = t * t;
    let uu = u * u;
    let uuu = uu * u;
    let ttt = tt * t;
    let Q = uuu * P0 + 3.0 * uu * t * P1 + 3.0 * u * tt * P2 + ttt * P3;
    return Q;
}

fn cubicBezierDerivative(P0 : vec2f, P1 : vec2f, P2 : vec2f, P3 : vec2f, t : f32) -> vec2f {
    return -3.0 * pow(1.0 - t, 2.0) * P0 + 
           (3.0 * pow(1.0 - t, 2.0) - 6.0 * t * (1.0 - t)) * P1 + 
           (6.0 * t * (1.0 - t) - 3.0 * t * t) * P2 + 
           3.0 * t * t * P3;
}

fn findClosestT(P0 : vec2f, P1 : vec2f, P2 : vec2f, P3 : vec2f, target_: vec2f, epsilon : f32, maxIterations : u32) -> f32 {
    var t = 0.5; // Initial guess
    for(var i: u32 = 0u; i < maxIterations; i = i + 1u) {
        let Q = cubicBezier(P0, P1, P2, P3, t);
        let Qprime = cubicBezierDerivative(P0, P1, P2, P3, t);
        let numerator = dot(Q - target_, Qprime);
        let denominator = dot(Qprime, Qprime);
        if (abs(numerator) < epsilon || denominator == 0.0) {
            break;
        }
        t -= numerator / denominator;
    }

    return min(1.0, max(0.0, t));
}

@compute @workgroup_size(1, 1)
fn main(@builtin(global_invocation_id) i : vec3<u32>) {
  let bezier = bezierBuffer.values[i.x];

  let point_a = vec3f(pointBuffer.values[bezier.point_a].px, pointBuffer.values[bezier.point_a].py, pointBuffer.values[bezier.point_a].pz);
  let point_b = vec3f(pointBuffer.values[bezier.point_b].px, pointBuffer.values[bezier.point_b].py, pointBuffer.values[bezier.point_b].pz);
  let control_a = vec3f(pointBuffer.values[bezier.control_a].px, pointBuffer.values[bezier.control_a].py, pointBuffer.values[bezier.control_a].pz);
  let control_b = vec3f(pointBuffer.values[bezier.control_b].px, pointBuffer.values[bezier.control_b].py, pointBuffer.values[bezier.control_b].pz);

  let p_a = to_screen_position(point_a);
  let p_b = to_screen_position(point_b);
  let c_a = to_screen_position(control_a);
  let c_b = to_screen_position(control_b);

  let t = findClosestT(p_a, c_a, c_b, p_b, vec2(uniforms.mouse_x, uniforms.height - uniforms.mouse_y), 1.0, 30u);
  let position = cubicBezier(p_a, c_a, c_b, p_b, t);
  let d = distance(position, vec2(uniforms.mouse_x, uniforms.height - uniforms.mouse_y));

  if(d <= 20.){
    bezierBuffer.values[i.x].flags = bezierBuffer.values[i.x].flags | 2;
  } else {
    bezierBuffer.values[i.x].flags = bezierBuffer.values[i.x].flags & (~2);
  }

}
@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(1) @binding(0) var<storage, read_write> hoverCounter : AtomicCounter;
@group(1) @binding(1) var<storage, read_write> hoverBuffer : HoverBuffer;
@group(2) @binding(0) var<storage, read_write> pointBuffer : PointBuffer;
@group(2) @binding(1) var<storage, read_write> bezierBuffer : BezierBuffer;

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

  let mouse_pos = vec2(uniforms.mouse_x, uniforms.mouse_y);

  let t = findClosestT(p_a, c_a, c_b, p_b, mouse_pos, 1.0, 30u);
  let position = cubicBezier(p_a, c_a, c_b, p_b, t);
  let d = distance(position, mouse_pos);

  if(d <= 20.){
    bezierBuffer.values[i.x].flags = bezierBuffer.values[i.x].flags | 2;

    let pos = get_position(point_a, control_a, control_b, point_b, t);
    let hover_index = atomicAdd(&hoverCounter.counter, 1u);
    hoverBuffer.values[hover_index].index = i.x;
    hoverBuffer.values[hover_index].ctype = 4; // bezier
    hoverBuffer.values[hover_index].distance = distance(pos, vec3f(uniforms.camera_origin_x, uniforms.camera_origin_y, uniforms.camera_origin_z));
    hoverBuffer.values[hover_index].position_x = pos.x;
    hoverBuffer.values[hover_index].position_y = pos.y;
    hoverBuffer.values[hover_index].position_z = pos.z;
  } else {
    bezierBuffer.values[i.x].flags = bezierBuffer.values[i.x].flags & (~2);
  }

}
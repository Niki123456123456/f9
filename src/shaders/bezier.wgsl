struct PointBuffer {
  values: array<Point>,
};
struct BezierBuffer {
  values: array<Bezier>,
};
struct VertexOutput {
  @builtin(position) position : vec4f,
  @location(0) @interpolate(flat) flags : i32,
};

@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(0) @binding(4) var<storage, read> pointBuffer : PointBuffer;
@group(0) @binding(6) var<storage, read> bezierBuffer : BezierBuffer;

fn get_position(t : f32, point_a : vec3f, point_b : vec3f, control_a : vec3f, control_b : vec3f) -> vec3f{
    let position = pow(1.0 - t, 3.0) * point_a
        + 3.0 * pow(1.0 - t, 2.0) * t * control_a
        + 3.0 * (1.0 - t) * pow(t, 2.0) * control_b
        + pow(t, 3.0) * point_b;
    return position;
}

@vertex
fn vert_main(@builtin(vertex_index) i : u32) -> VertexOutput {
  let bezier = bezierBuffer.values[i / u32(51)];
  let point_a = vec3f(pointBuffer.values[bezier.point_a].px, pointBuffer.values[bezier.point_a].py, pointBuffer.values[bezier.point_a].pz);
  let point_b = vec3f(pointBuffer.values[bezier.point_b].px, pointBuffer.values[bezier.point_b].py, pointBuffer.values[bezier.point_b].pz);
  let control_a = vec3f(pointBuffer.values[bezier.control_a].px, pointBuffer.values[bezier.control_a].py, pointBuffer.values[bezier.control_a].pz);
  let control_b = vec3f(pointBuffer.values[bezier.control_b].px, pointBuffer.values[bezier.control_b].py, pointBuffer.values[bezier.control_b].pz);

  let t = f32(i % u32(51)) / 50.0;

  var output : VertexOutput;
  output.position = uniforms.matrix * vec4f(get_position(t, point_a, point_b, control_a, control_b), 1.0);
  output.flags = bezier.flags;
  return output;
}

@fragment
fn frag_main(v: VertexOutput) -> @location(0) vec4f {
  var color = vec4f(1.0, 1.0, 1.0, 1.0);
  if ((v.flags & 2) == 2){ // hover
    color =  vec4f(1.0, 0.0, 0.0, 1.0);
  }
  return color;
}
struct Uniforms {
  width : f32,
  height : f32,
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
@group(0) @binding(4) var<storage, read> pointBuffer : PointBuffer;

@vertex
fn vert_main(@builtin(vertex_index) i : u32) -> @builtin(position) vec4f {
    let point = pointBuffer.values[i / u32(6)];
    let center = uniforms.matrix * vec4f(point.px, point.py, point.pz, 1.0);

    let size = vec2f(uniforms.width, uniforms.height);

    let vCenter = vec4f(((center.xyz/center.w) * 0.5 + 0.5) * vec3f(size.xy, 1.0), center.w).xy;

    let radius = 10.0;

    let diff_x = vec2f(radius, 0.0);
    let diff_y = vec2f(0.0, radius);
    let point_a = vec4f(center.w * ((vCenter - diff_x - diff_y)/size.xy - 0.5) / 0.5, center.z, center.w);
    let point_b = vec4f(center.w * ((vCenter + diff_x + diff_y)/size.xy - 0.5) / 0.5, center.z, center.w);
    let point_c = vec4f(center.w * ((vCenter - diff_x + diff_y)/size.xy - 0.5) / 0.5, center.z, center.w);
    let point_d = vec4f(center.w * ((vCenter + diff_x - diff_y)/size.xy - 0.5) / 0.5, center.z, center.w);

    var pos = array<vec4f, 6>();
    pos[0] = point_a;
    pos[1] = point_c;
    pos[2] = point_b;
    pos[3] = point_a;
    pos[4] = point_b;
    pos[5] = point_d;
    return pos[i % u32(6)];
}

@fragment
fn frag_main(@builtin(position) coord: vec4f) -> @location(0) vec4f {
  let color = vec4f(1.0, 1.0, 1.0, 1.0);
  return color;
}
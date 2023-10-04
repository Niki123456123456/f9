struct Uniforms {
  width : f32,
  height : f32,
  camera_orientation_x : f32,
  camera_orientation_y : f32,
  camera_orientation_z : f32,
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

struct VertexOutput {
  @builtin(position) position : vec4f,
  @location(0) vCenter : vec2f,
};

@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(0) @binding(4) var<storage, read> pointBuffer : PointBuffer;

@vertex
fn vert_main(@builtin(vertex_index) i : u32) -> VertexOutput {
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
    //return pos[i % u32(6)];

    var output : VertexOutput;
    output.position = pos[i % u32(6)];
    output.vCenter = vCenter;
    return output;
}

@fragment
fn frag_main( v: VertexOutput) -> @location(0) vec4f {
    var color = vec4f(1.0, 1.0, 1.0, 1.0);

    let distance = distance(v.position.xy - vec2f(0.0, 18.0), v.vCenter);
    if (distance > 10.0){
        color = vec4f(0.0, 0.0, 0.0, 0.0);
    }
    
    return color;
}
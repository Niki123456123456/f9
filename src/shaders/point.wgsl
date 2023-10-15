struct VertexOutput {
  @builtin(position) position : vec4f,
  @location(0) vCenter : vec2f,
  @location(1) @interpolate(flat) flags : i32,
};

@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(1) @binding(0) var<storage, read> pointBuffer : PointBuffer;

@vertex
fn vert_main(@builtin(vertex_index) i : u32) -> VertexOutput {
    let point = pointBuffer.values[i / u32(6)];
    let center = uniforms.matrix * vec4f(point.px, point.py, point.pz, 1.0);

    let size = vec2f(uniforms.width, uniforms.height);

    let vCenter = vec4f(((center.xyz/center.w) * 0.5 + 0.5) * vec3f(size, 1.0), center.w).xy;

    let radius = 5.0;

    let diff_x = vec2f(radius, 0.0);
    let diff_y = vec2f(0.0, radius);
    let point_a = vCenter - diff_x - diff_y;
    let point_b = vCenter + diff_x + diff_y;
    let point_c = vCenter - diff_x + diff_y;
    let point_d = vCenter + diff_x - diff_y;

    var pos = array<vec2f, 6>();
    pos[0] = point_a;
    pos[1] = point_c;
    pos[2] = point_b;
    pos[3] = point_a;
    pos[4] = point_b;
    pos[5] = point_d;

    var output : VertexOutput;
    output.position = vec4f(center.w * ((pos[i % u32(6)])/size - 0.5) / 0.5, center.z, center.w);
    output.vCenter = vec2f(vCenter.x, uniforms.height - vCenter.y);
    output.flags = point.flags;
    return output;
}

@fragment
fn frag_main( v: VertexOutput) -> @location(0) vec4f {
    var color = vec4f(1.0, 1.0, 1.0, 1.0);

    if ((v.flags & 2) == 2){ // hover
      color = vec4f(1.0, 0.0, 0.0, 1.0);
    }

    let distance = distance(v.position.xy - vec2f(0.0, uniforms.height_top), v.vCenter);
    if (distance > 5.0){
      color = vec4f(0.0, 0.0, 0.0, 0.0);
    }
    
    return color;
}
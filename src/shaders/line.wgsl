struct VertexOutput {
  @builtin(position) position : vec4f,
  @location(0) @interpolate(flat) flags : i32,
};

@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(1) @binding(0) var<storage, read> pointBuffer : PointBuffer;
@group(1) @binding(1) var<storage, read> lineBuffer : LineBuffer;


@vertex
fn vert_main(@builtin(vertex_index) i : u32) -> VertexOutput {
  let line = lineBuffer.values[i / u32(2)];
  var pos = array<vec3f, 2>();
  pos[0] = vec3f(pointBuffer.values[line.point_a].px, pointBuffer.values[line.point_a].py, pointBuffer.values[line.point_a].pz);
  pos[1] = vec3f(pointBuffer.values[line.point_b].px, pointBuffer.values[line.point_b].py, pointBuffer.values[line.point_b].pz);

  var output : VertexOutput;
  output.position = uniforms.matrix * vec4f(pos[i % u32(2)], 1.0);
  output.flags = line.flags;
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
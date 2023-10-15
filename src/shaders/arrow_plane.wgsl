struct VertexOutput {
  @builtin(position) position : vec4f,
  @location(0) @interpolate(flat) flags : i32,
};

@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(1) @binding(0) var<storage, read> vertexBuffer : VertexBuffer;

@vertex
fn vert_main(@builtin(vertex_index) i : u32) -> VertexOutput {
    let arrow = vertexBuffer.values[i / u32(6)];

    let scale_factor = 1.0;
    let width = 0.4;

    let direction = vec3f(arrow.dx, arrow.dy, arrow.dz);
    let position = vec3f(arrow.px, arrow.py, arrow.pz);
    let offset = (shift(direction) + shift(shift(direction))) * scale_factor * 0.1;
    let origin = position + offset;
    
    let a = origin;
    let b = origin + (shift(direction) + shift(shift(direction))) * scale_factor * width;
    let c = origin + shift(direction) * scale_factor * width;
    let d = origin + shift(shift(direction)) * scale_factor * width;

    var pos = array<vec3f, 6>();
    pos[0] = a;
    pos[1] = c;
    pos[2] = b;
    pos[3] = a;
    pos[4] = b;
    pos[5] = d;

  var output : VertexOutput;
  output.position = uniforms.matrix * vec4f(pos[i % u32(6)], 1.0);
  output.flags = arrow.flags;
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
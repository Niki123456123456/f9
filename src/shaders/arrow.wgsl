struct VertexOutput {
  @builtin(position) position : vec4f,
  @location(0) @interpolate(flat) flags : i32,
};

@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(1) @binding(0) var<storage, read> vertexBuffer : VertexBuffer;


@vertex
fn vert_main(@builtin(vertex_index) i : u32) -> VertexOutput {
  let arrow = vertexBuffer.values[i / u32(6)];
  let a = vec3f(arrow.px, arrow.py, arrow.pz);
  let arrow_direction = vec3f(arrow.dx, arrow.dy, arrow.dz);
  let spacing = 0.1;
  let camera_orientation = vec3f(uniforms.camera_orientation_x, uniforms.camera_orientation_y, uniforms.camera_orientation_z);

  let p1 = a + arrow_direction * (1. - spacing);
  let p2 = a + arrow_direction * spacing;
  let p0 = a + arrow_direction * (1. - 2. * spacing);
  let p3 = p0 + cross(arrow_direction, camera_orientation) * spacing;
  let p4 = p0 - cross(arrow_direction, camera_orientation) * spacing;

  var pos = array<vec3f, 6>();
  pos[0] = p1;
  pos[1] = p2;
  pos[2] = p1;
  pos[3] = p3;
  pos[4] = p1;
  pos[5] = p4;

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

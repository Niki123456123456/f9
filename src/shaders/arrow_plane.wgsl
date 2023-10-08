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
struct Vertex {
  px : f32,
  py : f32,
  pz : f32,
  dx : f32,
  dy : f32,
  dz : f32,
  flags : i32,
}
struct VertexBuffer {
  values: array<Vertex>,
};
struct VertexOutput {
  @builtin(position) position : vec4f,
  @location(0) flags : i32,
};

@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(0) @binding(8) var<storage, read> vertexBuffer : VertexBuffer;

fn shift(v : vec3f) -> vec3f{
    return vec3f(v.z, v.x, v.y);
}
fn double_shift(v : vec3f)-> vec3f{
    return vec3f(v.y, v.z, v.x);
}


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
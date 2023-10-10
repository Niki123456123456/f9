@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(0) @binding(2) var<storage, read> vertexBuffer : VertexBuffer;

@vertex
fn vert_main(@builtin(vertex_index) i : u32) -> @builtin(position) vec4f {
  let length = 10.0;
  let size = u32(11);
  let axis = vertexBuffer.values[i / u32(size * u32(4))];
  let p = vec3f(axis.px, axis.py, axis.pz);
  let d = vec3f(axis.dx, axis.dy, axis.dz);

  let i2 = i % u32(size * u32(2));
  let a = f32(i2 % u32(2));
  let b = f32(i2 / u32(2)) / f32(size - u32(1));

  var c1 = shift(d);
  var c2 = double_shift(d);

  if((i % u32(size * u32(4))) / (size * u32(2)) == u32(0)) {
    c1 = double_shift(d);
    c2 = shift(d);
  }

  let result = p 
    + c1 * a * length * 0.5
    - c1 * (1.0 - a) * length * 0.5
    + c2 * (b - 0.5) * length;
  return uniforms.matrix * vec4f(result, 1.0);
}

@fragment
fn frag_main(@builtin(position) coord: vec4f) -> @location(0) vec4f {
  let color = vec4f(1.0, 1.0, 1.0, 1.0);
  return color;
}
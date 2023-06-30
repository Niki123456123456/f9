struct ColorBuffer {
  values: array<atomic<u32>>,
};

struct UBO {
  screenWidth: f32,
  screenHeight: f32,
  modelViewProjectionMatrix: mat4x4<f32>,
 
};

struct Vertex { 
  //position : vec3<f32>,
  //direction : vec3<f32>,
  px : f32,
  py : f32,
  pz : f32,
  dx : f32,
  dy : f32,
  dz : f32,
 };

struct VertexBuffer {
  values: array<Vertex>,
};

@group(0) @binding(0) var<storage, read_write> outputColorBuffer : ColorBuffer;
@group(0) @binding(1) var<storage, read> vertexBuffer : VertexBuffer;
@group(0) @binding(2) var<uniform> uniforms : UBO;

fn draw_line(v1: vec2<f32>, v2: vec2<f32>) {
  let dist = i32(distance(v1, v2));
  for (var i = 0; i < dist; i = i + 1) {
    let x1 = v1.x + f32(v2.x - v1.x) * (f32(i) / f32(dist));
    let x2 = v1.y + f32(v2.y - v1.y) * (f32(i) / f32(dist));
    if (x1 > 0.0 && x2 > 0.0 && x1 < 2000.0 && x2 < 2000.0){
      let x = u32(x1);
      let y = u32(x2);
      set_pixel(x, y, 255u, 255u, 255u);
    }
  }
}

fn set_pixel(x : u32, y : u32, r : u32, g : u32, b : u32){
  atomicStore(&outputColorBuffer.values[(x + y * u32(uniforms.screenWidth)) * 3u + 0u], r);
  atomicStore(&outputColorBuffer.values[(x + y * u32(uniforms.screenWidth)) * 3u + 1u], g);
  atomicStore(&outputColorBuffer.values[(x + y * u32(uniforms.screenWidth)) * 3u + 2u], b);
}

@compute @workgroup_size(1, 1)
fn main(@builtin(global_invocation_id) global_id : vec3<u32>) {
  let step = 0.0001;
  let axis = vertexBuffer.values[global_id.x];
  let p = project(vec3<f32>(axis.px, axis.py, axis.pz)+ vec3<f32>(axis.dx, axis.dy, axis.dz) * step * f32(global_id.y));
  let p2 = project(vec3<f32>(axis.px, axis.py, axis.pz)+ vec3<f32>(axis.dx, axis.dy, axis.dz) * step * f32(global_id.y + 1u));

  if (is_off_screen(p) == false && is_off_screen(p2) == false) {
    draw_line(p, p2);
  }
  

}

fn is_off_screen(v: vec2<f32>) -> bool {
  if (v.x < 0.0 || v.x > uniforms.screenWidth || v.y < 0.0 || v.y > uniforms.screenHeight) {
    return true;
  }

  return false;
}

fn project(v : vec3<f32>) -> vec2<f32> {
  var pos = uniforms.modelViewProjectionMatrix * vec4<f32>(v.x, v.y, v.z, 1.0);
  pos.x = pos.x / pos.w * uniforms.screenWidth;
  pos.y = pos.y  / pos.w * uniforms.screenHeight;

  var size = vec2<f32>(uniforms.screenWidth, uniforms.screenHeight);

  return  vec4<f32>(((pos.xyz/pos.w) * 0.5 + 0.5) *  vec3<f32>(size, 1.0), pos.w).xy;
}

@compute @workgroup_size(256, 1)
fn clear(@builtin(global_invocation_id) global_id : vec3<u32>) {
  let index = global_id.x * 3u;

  atomicStore(&outputColorBuffer.values[index + 0u], 0u);
  atomicStore(&outputColorBuffer.values[index + 1u], 0u);
  atomicStore(&outputColorBuffer.values[index + 2u], 255u);
}
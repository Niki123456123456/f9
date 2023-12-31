struct Triangle2f{
  p0 : vec2f,
  p1 : vec2f,
  p2 : vec2f,
}

@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(1) @binding(0) var<storage, read_write> hoverCounter : AtomicCounter;
@group(1) @binding(1) var<storage, read_write> hoverBuffer : HoverBuffer;
@group(2) @binding(0) var<storage, read_write> vertexBuffer : VertexBuffer;

fn is_point_in(triangle : Triangle2f, point: vec2f) -> bool {
        // https://blackpawn.com/texts/pointinpoly/

        // Compute vectors
        let v0 = triangle.p2 - triangle.p0;
        let v1 = triangle.p1 - triangle.p0;
        let v2 = point - triangle.p0;

        // Compute dot products
        let dot00 = dot(v0, v0);
        let dot01 = dot(v0, v1);
        let dot02 = dot(v0, v2);
        let dot11 = dot(v1, v1);
        let dot12 = dot(v1, v2);

        // Compute barycentric coordinates
        let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
        let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
        let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

        // Check if point is in triangle
        return (u >= 0.0) && (v >= 0.0) && (u + v < 1.0);
    }

@compute @workgroup_size(1, 1)
fn main(@builtin(global_invocation_id) i : vec3<u32>) {
  let arrow = vertexBuffer.values[i.x];
  let scale_factor = 1.0;
  let width = 0.4;

  let direction = vec3f(arrow.dx, arrow.dy, arrow.dz);
  let position = vec3f(arrow.px, arrow.py, arrow.pz);
  let offset = (shift(direction) + shift(shift(direction))) * scale_factor * 0.1;
  let origin = position + offset;
    
  let a = to_screen_position(origin);
  let b = to_screen_position(origin + (shift(direction) + shift(shift(direction))) * scale_factor * width);
  let c = to_screen_position(origin + shift(direction) * scale_factor * width);
  let d = to_screen_position(origin + shift(shift(direction)) * scale_factor * width);

  var triangle0 : Triangle2f;
  triangle0.p0 = a;
  triangle0.p1 = c;
  triangle0.p2 = b;

  var triangle1 : Triangle2f;
  triangle1.p0 = a;
  triangle1.p1 = b;
  triangle1.p2 = d;

  let mouse_pos = vec2(uniforms.mouse_x, uniforms.mouse_y);

  if(is_point_in(triangle0, mouse_pos) || is_point_in(triangle1, mouse_pos)){
    vertexBuffer.values[i.x].flags = vertexBuffer.values[i.x].flags | 2;
    let hover_index = atomicAdd(&hoverCounter.counter, 1u);
    hoverBuffer.values[hover_index].index = i.x;
    hoverBuffer.values[hover_index].ctype = 6; // arrow plane
    hoverBuffer.values[hover_index].distance = 0.0;
    hoverBuffer.values[hover_index].position_x = 0.0;
    hoverBuffer.values[hover_index].position_y = 0.0;
    hoverBuffer.values[hover_index].position_z = 0.0;
  } else {
    vertexBuffer.values[i.x].flags = vertexBuffer.values[i.x].flags & (~2);
  }

}
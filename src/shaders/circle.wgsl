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
struct Point {
  px : f32,
  py : f32,
  pz : f32,
  flags : i32,
}
struct PointBuffer {
  values: array<Point>,
};
struct Circle {
  center: u32,
  radius: f32,
  orientation_x : f32,
  orientation_y : f32,
  orientation_z : f32,
  heightfactor: f32,
  flags : i32,
}
struct CircleBuffer {
  values: array<Circle>,
};
struct VertexOutput {
  @builtin(position) position : vec4f,
  @location(0) flags : i32,
};

@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(0) @binding(4) var<storage, read> pointBuffer : PointBuffer;
@group(0) @binding(7) var<storage, read> circleBuffer : CircleBuffer;

const PI: f32 = 3.1415926538;

fn rotateAroundAxis(point : vec3f, axisOrigin : vec3f, axisDirection : vec3f, angle : f32) -> vec3f {
    // Normiere die Achsenrichtung
    let normalizedAxisDirection = normalize(axisDirection);

    // Verschiebe den Punkt, so dass der Ursprung am Koordinatenursprung liegt
    let shiftedPoint = point - axisOrigin;

    // Rotiere den verschobenen Punkt um die Achse
    let cosAngle = cos(angle);
    let sinAngle = sin(angle);
    let rotatedPoint = cosAngle * shiftedPoint +
                        (1.0 - cosAngle) * dot(shiftedPoint, normalizedAxisDirection) * normalizedAxisDirection +
                        sinAngle * cross(normalizedAxisDirection, shiftedPoint);

    // Verschiebe den rotierten Punkt zurück zum ursprünglichen Ursprung
    let finalPoint = rotatedPoint + axisOrigin;

    return finalPoint;
}

fn orthogonal(v : vec3f) -> vec3f {
    if (abs(v.x) > abs(v.y)){
        return vec3f(-v.z, 0.0, v.x); // cross(v, y)
    } else {
        return vec3f(0.0, v.z, -v.y);  // cross(v, x)
    }
}


fn get_position(t : f32, center : vec3f, radius : f32, orientation : vec3f) -> vec3f{
    let x = (2.0 * PI * t);
    let a = center + orthogonal(orientation) * radius;
    let position = rotateAroundAxis(a, center, orientation, x);
    return position;
}

@vertex
fn vert_main(@builtin(vertex_index) i : u32) -> VertexOutput {
  let circle = circleBuffer.values[i / u32(51)];
  let t = f32(i % u32(51)) / 50.0;
  let center = vec3f(pointBuffer.values[circle.center].px, pointBuffer.values[circle.center].py, pointBuffer.values[circle.center].pz);
  let radius = circle.radius;
  let orientation = vec3f(circle.orientation_x, circle.orientation_y, circle.orientation_z);

  var output : VertexOutput;
  output.position = uniforms.matrix * vec4f(get_position(t, center, radius, orientation), 1.0);
  output.flags = circle.flags;
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
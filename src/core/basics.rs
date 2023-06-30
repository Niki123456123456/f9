use glam::{vec2, vec3, vec4, Mat4, Vec2, Vec3, Vec4};

const MAX: f32 = 1e-30;

struct Triangle {
    p0: Vec2,
    p1: Vec2,
    p2: Vec2,
}

pub struct Rec2 {
    pub p0: Vec2,
    pub p1: Vec2,
    pub p2: Vec2,
    pub p3: Vec2,
}

impl Rec2 {
    

    

    pub fn is_point_in(&self, point: Vec2) -> bool {
        let a = Triangle {
            p0: self.p0,
            p1: self.p1,
            p2: self.p2,
        };
        let b = Triangle {
            p0: self.p2,
            p1: self.p3,
            p2: self.p0,
        };

        return a.is_point_in(point) || b.is_point_in(point);
    }
}

impl Triangle {
    fn is_point_in(&self, point: Vec2) -> bool {
        // https://blackpawn.com/texts/pointinpoly/

        // Compute vectors
        let v0 = self.p2 - self.p0;
        let v1 = self.p1 - self.p0;
        let v2 = point - self.p0;

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
}

fn dot(a: Vec2, b: Vec2) -> f32 {
    a.x * b.x + a.y * b.y
}

pub fn rot90(v: Vec2) -> Vec2 {
    vec2(v.y, -v.x)
}



#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl BoundingBox {
    pub fn from_points(points: Vec<Vec3>) -> BoundingBox {
        let first = points.first().unwrap();
        let mut bounding = BoundingBox {
            min: first.clone(),
            max: first.clone(),
        };
        for point in points[1..].iter() {
            if point.x < bounding.min.x {
                bounding.min.x = point.x;
            }
            if point.y < bounding.min.y {
                bounding.min.y = point.y;
            }
            if point.z < bounding.min.z {
                bounding.min.z = point.z;
            }
            if point.x > bounding.max.x {
                bounding.max.x = point.x;
            }
            if point.y > bounding.max.y {
                bounding.max.y = point.y;
            }
            if point.z > bounding.max.z {
                bounding.max.z = point.z;
            }
        }

        return bounding;
    }
    pub fn intersect(&self, ray: &Ray) -> Option<Vec3> {
        let inv_dir_x = 1.0 / ray.direction.x;
        let inv_dir_y = 1.0 / ray.direction.y;
        let inv_dir_z = 1.0 / ray.direction.z;

        // Calculate distances to near and far planes in each axis
        let (tx_min, tx_max) = if inv_dir_x >= 0.0 {
            (
                (self.min.x - ray.origin.x) * inv_dir_x,
                (self.max.x - ray.origin.x) * inv_dir_x,
            )
        } else {
            (
                (self.max.x - ray.origin.x) * inv_dir_x,
                (self.min.x - ray.origin.x) * inv_dir_x,
            )
        };
        let (ty_min, ty_max) = if inv_dir_y >= 0.0 {
            (
                (self.min.y - ray.origin.y) * inv_dir_y,
                (self.max.y - ray.origin.y) * inv_dir_y,
            )
        } else {
            (
                (self.max.y - ray.origin.y) * inv_dir_y,
                (self.min.y - ray.origin.y) * inv_dir_y,
            )
        };
        let (tz_min, tz_max) = if inv_dir_z >= 0.0 {
            (
                (self.min.z - ray.origin.z) * inv_dir_z,
                (self.max.z - ray.origin.z) * inv_dir_z,
            )
        } else {
            (
                (self.max.z - ray.origin.z) * inv_dir_z,
                (self.min.z - ray.origin.z) * inv_dir_z,
            )
        };

        // Calculate largest minimum and smallest maximum distances for each axis
        let t_min = tx_min.max(ty_min).max(tz_min);
        let t_max = tx_max.min(ty_max).min(tz_max);

        // Check for intersection
        if t_max < 0.0 || t_min > t_max {
            None
        } else {
            Some(ray.origin + ray.direction * t_min)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Plane {
    pub position: Vec3,
    pub orientation: Vec3,
}

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}
#[derive(Debug, Clone)]
pub struct Line {
    pub a: Vec3,
    pub b: Vec3,
}

impl Line {
    pub fn to_ray(&self) -> Ray {
        Ray {
            origin: self.a,
            direction: (self.b - self.a).normalize(),
        }
    }

    pub fn direction(&self) -> Vec3 {
        self.b - self.a
    }

    pub fn position(&self, t: f32) -> Vec3 {
        if t < 0. {
            return self.a;
        } else if t > 1. {
            return self.b;
        } else {
            let closest_point_b = self.a + self.direction() * t;
            return closest_point_b;
        }
    }

    pub fn position_by_min_distance(&self, line: impl Into<Line>) -> Vec3 {
        let t = t_by_min_distance(self.clone(), line);
        return self.position(t);
    }
}

impl Ray {
    pub fn position(&self, t: f32) -> Vec3 {
        return self.origin + t * self.direction;
    }

    pub fn position_by_min_distance(&self, line: impl Into<Line>) -> Vec3 {
        let t = t_by_min_distance(self.clone(), line);
        return self.position(t);
    }
}

impl Into<Line> for Ray {
    fn into(self) -> Line {
        Line {
            a: self.origin,
            b: self.origin + self.direction,
        }
    }
}

pub fn t_by_min_distance(line: impl Into<Line>, line2: impl Into<Line>) -> f32 {
    let line: Line = line.into();
    let line2: Line = line2.into();

    let cross = line2
        .direction()
        .cross(line.direction())
        .cross(line2.direction())
        .normalize();
    let t = (line2.a - line.a).dot(cross) / line.direction().dot(cross);

    return t;
}

pub fn intersert(plane: &Plane, ray: &Ray) -> Vec3 {
    // https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-plane-and-ray-disk-intersection.html

    let denom = plane.orientation.dot(ray.direction);
    if denom > MAX || denom < MAX {
        let p0l0 = plane.position - ray.origin;
        let t = p0l0.dot(plane.orientation) / denom;
        return ray.position(t);
    }

    return ray.origin;
}

pub fn to_screen_position(projection: Mat4, size: Vec2, position: Vec3) -> Vec2 {
    let pos = projection * vec4(position.x, position.y, position.z, 1.);
    let result = ((vec3(pos.x, pos.y, pos.z) / pos.w) * 0.5 + 0.5) * vec3(size.x, size.y, 1.0);
    return vec2(result.x, size.y - result.y);
}

pub fn perp(a: Vec2, b: Vec2, c: Vec2) -> f32 {
    // perpendicular distance between line a+t*b and point c
    let distance = (c - a - (c - a).dot(b) * b).length();
    return distance;
}

pub fn shift(v: Vec3) -> Vec3 {
    vec3(v.z, v.x, v.y)
}

pub fn double_shift(v: Vec3) -> Vec3 {
    vec3(v.y, v.z, v.x)
}

pub fn power(f: f32, power: i32) -> f32 {
    let mut result = 1.0;
    for i in 0..power {
        result = result * f;
    }
    return result;
}
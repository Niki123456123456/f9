use glam::Vec3;
use crate::component_collection::ComponentCollection;

use super::component::{Component, IComponentData, IComponent};

#[derive(Clone)]
#[repr(C)]
pub struct Line {
    pub point_a: u32,
    pub point_b: u32,
}

impl IComponentData for Line {
    fn get_center(& self, components : &ComponentCollection) -> Vec3 {
        let a = components.points.array[self.point_a as usize].get_center(components);
        let b = components.points.array[self.point_b as usize].get_center(components);
        return a + (b - a) / 2.;
    }
}

pub fn new(point_a: u32, point_b: u32) -> Component<Line> {
    Component::new(Line { point_a, point_b })
}
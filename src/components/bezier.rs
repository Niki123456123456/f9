use crate::component_collection::ComponentCollection;
use glam::Vec3;

use super::component::{Component, IComponent, IComponentData};

#[derive(Clone)]
#[repr(C)]
pub struct Bezier {
    pub point_a: u32,
    pub point_b: u32,
    pub control_a: u32,
    pub control_b: u32,
}

impl IComponentData for Bezier {
    fn get_center(&self, components: &ComponentCollection) -> Vec3 {
        Vec3::ZERO
    }
}

pub fn new(point_a: u32, point_b: u32, control_a: u32, control_b: u32) -> Component<Bezier> {
    Component::new(Bezier {
        point_a,
        point_b,
        control_a,
        control_b,
    })
}

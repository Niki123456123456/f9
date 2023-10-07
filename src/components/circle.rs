use crate::component_collection::ComponentCollection;
use glam::Vec3;

use super::component::{Component, IComponent, IComponentData};

#[derive(Clone)]
#[repr(C)]
pub struct Circle {
    pub center: u32,
    pub radius: f32,
    pub orientation: Vec3,
    pub heightfactor: f32,
}

impl IComponentData for Circle {
    fn get_center(&self, components: &ComponentCollection) -> Vec3 {
        components.points.array[self.center as usize].get_center(components)
    }
}

pub fn new(center: u32, radius: f32, orientation: Vec3, heightfactor: f32) -> Component<Circle> {
    Component::new(Circle {
        center,
        radius,
        orientation,
        heightfactor,
    })
}
use glam::Vec3;
use crate::component_collection::ComponentCollection;

use super::component::{Component, IComponentData};

#[derive(Clone)]
#[repr(C)]
pub struct Vertex {
    pub position: Vec3,
    pub direction: Vec3,
}

impl IComponentData for Vertex {
    fn get_center(& self, components : &ComponentCollection) -> Vec3 {
        self.position
    }
}

impl Vertex {
    pub fn new(position: Vec3, direction: Vec3) -> Self {
        Self {
            position,
            direction,
        }
    }

    pub fn zero(direction: Vec3) -> Self {
        Self {
            position: Vec3::ZERO,
            direction,
        }
    }
}

pub fn x() -> Component<Vertex> {
    Component::new(Vertex::zero(Vec3::X))
}

pub fn y() -> Component<Vertex> {
    Component::new(Vertex::zero(Vec3::Y))
}

pub fn z() -> Component<Vertex> {
    Component::new(Vertex::zero(Vec3::Z))
}


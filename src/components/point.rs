use glam::Vec3;
use crate::component_collection::ComponentCollection;

use super::component::{Component, IComponentData};

#[derive(Clone)]
#[repr(C)]
pub struct Point {
    pub position: Vec3,
}

impl IComponentData for Point {
    fn get_center(& self, components : &ComponentCollection) -> Vec3 {
        self.position
    }
}

impl From<Vec3> for Point {
    fn from(vec: Vec3) -> Self {
        Point { position: vec }
    }
}

impl Into<Vec3> for Point {
    fn into(self) -> Vec3 {
        self.position
    }
}

pub fn new(position: Vec3) -> Component<Point> {
    Component::new(position.into())
}
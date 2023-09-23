use std::sync::Arc;

use eframe::wgpu::{self, Device};

use crate::{
    camera::Camera,
    component_collection::{ComponentArray, ComponentCollection},
    components::vertex,
};

pub struct ProjectState {
    pub camera: Camera,
    pub components: ComponentCollection,
}

pub struct Project {
    pub name: String,
    pub state: ProjectState,
}

impl Project {
    pub fn new(device: &Arc<Device>, queue: &wgpu::Queue) -> Project {
        let mut axises = vec![];
        axises.push(vertex::x().notvisible());
        axises.push(vertex::x());
        axises.push(vertex::y().notvisible());
        axises.push(vertex::z());

        Self {
            name: "New Project".into(),
            state: ProjectState {
                camera: Camera::default(),
                components: ComponentCollection {
                    axises: ComponentArray::new(axises, device, queue),
                },
            },
        }
    }
}

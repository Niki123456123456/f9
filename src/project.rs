use std::sync::Arc;

use eframe::wgpu::{self, Device};
use glam::vec3;

use crate::{
    camera::Camera,
    component_collection::{ComponentArray, ComponentCollection},
    components::{vertex, point},
    rendering::{
        buffer::UniformBuffer,
        renderer::{get_layout, uniform, storage},
    },
};

pub struct ProjectState {
    pub camera: Camera,
    pub components: ComponentCollection,
    pub uniform_buffer: Arc<UniformBuffer>,
}

pub struct Project {
    pub name: String,
    pub state: ProjectState,
}

impl Project {
    pub fn new(device: &Arc<Device>, queue: &wgpu::Queue) -> Project {
        let axises = ComponentArray::new(
            vec![
                vertex::x().notvisible(),
                vertex::x(),
                vertex::y().notvisible(),
                vertex::z(),
            ],
            device,
            queue,
        );
        let grids = ComponentArray::new(
            vec![
                vertex::x().notvisible(),
                vertex::x().notvisible(),
                vertex::y(),
                vertex::z().notvisible(),
            ],
            device,
            queue,
        );
        let arrows = ComponentArray::new(
            vec![
                vertex::x().notvisible(),
                vertex::y().notvisible(),
                vertex::z().notvisible(),
            ],
            device,
            queue,
        );
        let points = ComponentArray::new(
            vec![
                point::new(vec3(0.0, 0.0, 0.0))
            ],
            device,
            queue,
        );

        let layout = get_layout(device, &[uniform(0), storage(1), storage(2), storage(3), storage(4)]);
        let buffer = UniformBuffer::new(device, &layout, 4 * 5+4 + 4 * 16 + 8, vec![&axises.buffer, &grids.buffer, &arrows.buffer, &points.buffer]);
        Self {
            name: "New Project".into(),
            state: ProjectState {
                camera: Camera::default(),
                components: ComponentCollection { axises, grids, arrows, points },
                uniform_buffer: Arc::new(buffer),
            },
        }
    }
}

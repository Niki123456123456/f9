use std::sync::Arc;

use eframe::wgpu::{self, Device};

use crate::{
    camera::Camera,
    component_collection::{ComponentArray, ComponentCollection},
    components::vertex,
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

        let layout = get_layout(device, &[uniform(0), storage(1), storage(2)]);
        let buffer = UniformBuffer::new(device, &layout, 4 * 2 + 4 * 16 + 8, vec![&axises.buffer, &grids.buffer]);
        Self {
            name: "New Project".into(),
            state: ProjectState {
                camera: Camera::default(),
                components: ComponentCollection { axises, grids },
                uniform_buffer: Arc::new(buffer),
            },
        }
    }
}

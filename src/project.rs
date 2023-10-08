use std::sync::Arc;

use eframe::wgpu::{self, Device};
use glam::{vec3, Vec2};

use crate::{
    camera::Camera,
    component_collection::{ComponentArray, ComponentCollection},
    components::{line, point, vertex, bezier, circle},
    rendering::{
        buffer::UniformBuffer,
        renderer::{get_layout, storage, uniform, storage_writeable},
    },
};

pub struct ProjectState {
    pub camera: Camera,
    pub components: ComponentCollection,
    pub uniform_buffer: Arc<UniformBuffer>,
    pub hover_pos : Vec2,
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
                point::new(vec3(0.0, 0.0, 0.0)),
                point::new(vec3(1.0, 1.0, 0.0)),
                point::new(vec3(1.0, 0.0, 0.0)),
                point::new(vec3(0.0, 1.0, 0.0)),
            ],
            device,
            queue,
        );

        let circles = ComponentArray::new(
            vec![
                circle::new(0, 2.5, vec3(1.0, 0.0, 0.0), 0.0)
            ],
            device,
            queue,
        );

        let lines = ComponentArray::new(vec![line::new(0, 1)], device, queue);

        let beziers = ComponentArray::new(vec![bezier::new(0, 3, 2, 1)], device, queue);

        let arrow_planes = ComponentArray::new(
            vec![
                vertex::x().notvisible(),
                vertex::y().notvisible(),
                vertex::z().notvisible(),
            ],
            device,
            queue,
        );

        let layout = get_layout(
            device,
            &[
                uniform(0),
                storage(1),
                storage(2),
                storage(3),
                storage(4),
                storage(5),
                storage(6),
                storage(7),
                storage(8),
            ],
        );

        let compute_layout = get_layout(
            device,
            &[
                uniform(0),
                storage_writeable(1),
                storage_writeable(2),
                storage_writeable(3),
                storage_writeable(4),
                storage_writeable(5),
                storage_writeable(6),
                storage_writeable(7),
                storage_writeable(8),
            ],
        );

        let buffer = UniformBuffer::new(
            device,
            &layout, &compute_layout,
            4 * 16 + 4 * 16,
            vec![
                &axises.buffer,
                &grids.buffer,
                &arrows.buffer,
                &points.buffer,
                &lines.buffer,
                &beziers.buffer,
                &circles.buffer,
                &arrow_planes.buffer,
            ],
        );
        Self {
            name: "New Project".into(),
            state: ProjectState {
                camera: Camera::default(),
                components: ComponentCollection {
                    axises,
                    grids,
                    arrows,
                    arrow_planes,
                    points,
                    lines,
                    beziers,
                    circles,
                },
                uniform_buffer: Arc::new(buffer),
                hover_pos: Vec2::ZERO,
            },
        }
    }
}

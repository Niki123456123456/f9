use std::{collections::HashMap, sync::Arc};

use async_channel::{Receiver, Sender};
use eframe::wgpu::{self, Device, Queue};
use glam::{vec3, Vec2};
use uuid::Uuid;

use crate::{
    camera::Camera,
    component_collection::{ComponentArray, ComponentCollection},
    components::{bezier, circle, line, point, vertex},
    dispatchers::dispatcher::{Disp, DispatcherEvent},
    rendering::{
        buffer::UniformBuffer,
        renderer::{get_layout, storage, storage_writeable, uniform, self, Renderer},
    },
};

pub struct ProjectState {
    pub camera: Camera,
    pub components: ComponentCollection,
    pub uniform_buffer: Arc<UniformBuffer>,
    pub hover_pos: Vec2,

    pub is_mouse_clicked: bool,
}

pub struct Project {
    pub name: String,
    pub state: ProjectState,

    pub dispatchers: HashMap<Uuid, Disp>,
    pub sender: Sender<DispatcherEvent>,
    pub receiver: Receiver<DispatcherEvent>,
}

impl Project {
    pub fn new(device: &Arc<Device>, queue: &Arc<Queue>, renderer : &Renderer) -> Project {
        let axises = ComponentArray::new(
            vec![
                vertex::x().notvisible(),
                vertex::x(),
                vertex::y().notvisible(),
                vertex::z(),
            ],
            device, queue,
        );
        let grids = ComponentArray::new(
            vec![
                vertex::x().notvisible(),
                vertex::x().notvisible(),
                vertex::y(),
                vertex::z().notvisible(),
            ],
            device, queue,
        );
        let arrows = ComponentArray::new(
            vec![
                vertex::x().notvisible(),
                vertex::y().notvisible(),
                vertex::z().notvisible(),
            ],
            device, queue,
        );
        let points = ComponentArray::new(
            vec![
                point::new(vec3(0.0, 0.0, 0.0)),
                point::new(vec3(1.0, 1.0, 0.0)),
                point::new(vec3(1.0, 0.0, 0.0)),
                point::new(vec3(0.0, 1.0, 0.0)),
            ],
            device, queue,
        );

        let circles =
            ComponentArray::new(vec![circle::new(0, 2.5, vec3(1.0, 0.0, 0.0), 0.0)], device, queue,);

        let lines = ComponentArray::new(vec![line::new(0, 1)], device, queue,);

        let beziers = ComponentArray::new(vec![bezier::new(0, 3, 2, 1)], device, queue,);

        let arrow_planes = ComponentArray::new(
            vec![
                vertex::x().notvisible(),
                vertex::y().notvisible(),
                vertex::z().notvisible(),
            ],
            device, queue,
        );

        let components = ComponentCollection {
            axises,
            grids,
            arrows,
            arrow_planes,
            points,
            lines,
            beziers,
            circles,
            hovers: vec![],
            selected: vec![],
        };

        let buffer = UniformBuffer::new(device, 4 * 16 + 4 * 16, &components, renderer);

        let (s, r): (Sender<DispatcherEvent>, Receiver<DispatcherEvent>) =
            async_channel::unbounded();

        Self {
            dispatchers: HashMap::new(),
            sender: s,
            receiver: r,

            name: "New Project".into(),
            state: ProjectState {
                camera: Camera::default(),
                components,
                uniform_buffer: Arc::new(buffer),
                hover_pos: Vec2::ZERO,

                is_mouse_clicked: false,
            },
        }
    }
}

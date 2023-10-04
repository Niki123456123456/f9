use std::{array, sync::Arc};

use eframe::wgpu::{self, BufferUsages, Device};

use crate::components::{component::Component, vertex::Vertex, point::Point};

pub struct ComponentCollection {
    pub axises: ComponentArray<Vertex>,
    pub grids: ComponentArray<Vertex>,
    pub arrows: ComponentArray<Vertex>,
    pub points: ComponentArray<Point>,
}

pub struct ComponentArray<T> {
    pub array: Vec<Component<T>>,
    pub buffer_size: usize,
    pub buffer: wgpu::Buffer,
}

impl<T> ComponentArray<T> {
    pub fn new(
        array: Vec<Component<T>>,
        device: &Arc<Device>,
        queue: &wgpu::Queue,
    ) -> ComponentArray<T> {
        let mem_size =core::mem::size_of::<Component<T>>();
        let buffer_size = array.len() * mem_size;

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
            mapped_at_creation: false,
            size: buffer_size as u64,
        });

        unsafe {
            let data: &[u8] = core::slice::from_raw_parts(
                array.as_ptr() as *const u8,
                array.len() * mem_size,
            );
            queue.write_buffer(&buffer, 0, &data);
        }
       

        return ComponentArray {
            array,
            buffer_size,
            buffer,
        };
    }
}

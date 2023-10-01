use std::{array, sync::Arc};

use eframe::wgpu::{self, BufferUsages, Device};

use crate::components::{component::Component, vertex::Vertex};

pub struct ComponentCollection {
    pub axises: ComponentArray<Vertex>,
}

pub struct ComponentArray<T> {
    pub array: Vec<Component<T>>,
    pub buffer_size: usize,
    pub buffer: wgpu::Buffer,
}

impl<T> ComponentArray<T> {
    pub fn new(
        data: Vec<Component<T>>,
        device: &Arc<Device>,
        queue: &wgpu::Queue,
    ) -> ComponentArray<T> {
        let buffer_size = data.len() * core::mem::size_of::<Component<T>>();

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
            mapped_at_creation: false,
            size: buffer_size as u64,
        });

        unsafe {
            let data: &[u8] = core::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * core::mem::size_of::<T>(),
            );
            queue.write_buffer(&buffer, 0, bytemuck::cast_slice(&data));
        }

        return ComponentArray {
            array: data,
            buffer_size,
            buffer,
        };
    }
}

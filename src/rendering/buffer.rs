use std::{collections::HashMap, sync::Arc};

use eframe::wgpu::{
    self, BindGroupDescriptor, BindGroupLayout, Buffer, BufferDescriptor, BufferUsages, Device,
};
use glam::Mat4;

use crate::{component_collection::ComponentCollection, components::component::HoverElement};

use super::renderer::{get_layout, storage_writeable, uniform, Renderer};

pub struct UniformBuffer {
    pub device: Arc<Device>,
    pub uniform_buffer: wgpu::Buffer,
    pub atomic_buffer: wgpu::Buffer,
    pub hover_buffer: wgpu::Buffer,

    pub uniform_bind_group: wgpu::BindGroup,
    pub hover_bind_group: wgpu::BindGroup,
    pub bind_groups: HashMap<String, wgpu::BindGroup>,
}

impl UniformBuffer {
    pub fn new(device: &Arc<Device>, size: u64, components: &ComponentCollection, renderer : &Renderer) -> Self {
        let uniform_buffer = device.create_buffer(&BufferDescriptor {
            label: None,
            size,
            usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });

        let atomic_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("atomic_buffer"),
            size: 32,
            usage: BufferUsages::COPY_DST | BufferUsages::COPY_SRC | BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        let hover_buffer = device.create_buffer(&BufferDescriptor {
            label: None,
            size: 1000 * core::mem::size_of::<HoverElement>() as u64,
            usage: BufferUsages::COPY_DST | BufferUsages::COPY_SRC | BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        let uniform_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &get_layout(device, &[uniform(0)]),
            entries: &vec![wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let hover_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &get_layout(device, &[storage_writeable(0), storage_writeable(1)]),
            entries: &vec![
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: atomic_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: hover_buffer.as_entire_binding(),
                },
            ],
        });

        let mut bind_groups = HashMap::new();
        for shader in renderer.shaders.iter() {
            bind_groups.insert(shader.label.to_string(), shader.get_bindgroup(device, components));
        }
        for shader in renderer.compute_shaders.iter() {
            bind_groups.insert(shader.label.to_string(), shader.get_bindgroup(device, components));
        }

        return Self {
            bind_groups,
            uniform_bind_group,
            uniform_buffer,
            atomic_buffer,
            hover_bind_group,
            hover_buffer,
            device: device.clone(),
        };
    }

    pub fn clear_hover_counter(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.atomic_buffer, 0, &[0, 0, 0, 0]);
    }

    pub fn write<A: bytemuck::NoUninit>(&self, queue: &wgpu::Queue, offset: u64, a: &[A]) {
        queue.write_buffer(&self.uniform_buffer, offset, bytemuck::cast_slice(a));
    }

    pub fn write_mat(&self, queue: &wgpu::Queue, offset: u64, a: &Mat4) {
        let mx_ref: &[f32; 16] = a.as_ref();

        queue.write_buffer(&self.uniform_buffer, offset, bytemuck::cast_slice(mx_ref));
    }
}

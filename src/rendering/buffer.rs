use std::sync::Arc;

use eframe::wgpu::{
    self, BindGroupDescriptor, BindGroupLayout, Buffer, BufferDescriptor, BufferUsages, Device,
};
use glam::Mat4;

use crate::components::component::HoverElement;

use super::renderer::{get_layout, storage_writeable};

pub struct UniformBuffer {
    pub bind_group: wgpu::BindGroup,
    pub compute_bind_group: wgpu::BindGroup,
    pub atomic_bind_group: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
    pub atomic_buffer: wgpu::Buffer,
    pub hover_buffer: wgpu::Buffer,
    pub device: Arc<Device>,
}

impl UniformBuffer {
    pub fn new(
        device: &Arc<Device>,
        layout: &BindGroupLayout,
        compute_layout: &BindGroupLayout,
        size: u64,
        buffers: Vec<&Buffer>,
    ) -> Self {
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

        let mut bind_group_entries = vec![wgpu::BindGroupEntry {
            binding: 0,
            resource: uniform_buffer.as_entire_binding(),
        }];
        for (i, buffer) in buffers.iter().enumerate() {
            bind_group_entries.push(wgpu::BindGroupEntry {
                binding: i as u32 + 1,
                resource: buffer.as_entire_binding(),
            })
        }

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout,
            entries: &bind_group_entries,
        });

        let compute_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: compute_layout,
            entries: &bind_group_entries,
        });

        let atomic_bind_group = device.create_bind_group(&BindGroupDescriptor {
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

        return Self {
            bind_group,
            compute_bind_group,
            uniform_buffer,
            atomic_buffer,
            atomic_bind_group,
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

use std::sync::Arc;

use eframe::wgpu::{self, BufferDescriptor, BindGroupDescriptor, Device, BufferUsages, BindGroupLayout, Buffer};
use glam::Mat4;

pub struct UniformBuffer{
    pub bind_group: wgpu::BindGroup,
    pub compute_bind_group: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
}

impl UniformBuffer {
    pub fn new(device: &Arc<Device>, layout : &BindGroupLayout, compute_layout : &BindGroupLayout, size : u64, buffers: Vec<& Buffer>) -> Self{
    let uniform_buffer = device.create_buffer(&BufferDescriptor {
        label: None,
        size,
        usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
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

    return Self{ bind_group, compute_bind_group, uniform_buffer};
    }

    pub fn write<A: bytemuck::NoUninit>(&self, queue: &wgpu::Queue, offset : u64, a: &[A]){
        queue.write_buffer(
            &self.uniform_buffer,
            offset,
            bytemuck::cast_slice(a),
        );
    }

    pub fn write_mat(&self, queue: &wgpu::Queue, offset : u64, a: &Mat4){
        let mx_ref: &[f32; 16] = a.as_ref();

        queue.write_buffer(
            &self.uniform_buffer,
            offset,
            bytemuck::cast_slice(mx_ref),
        );
    }

}
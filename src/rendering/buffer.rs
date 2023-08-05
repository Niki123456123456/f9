use std::sync::Arc;

use eframe::wgpu::{self, BufferDescriptor, BindGroupDescriptor, Device, BufferUsages, BindGroupLayout};
use glam::Mat4;

pub struct UniformBuffer{
    bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
}

impl UniformBuffer {
    pub fn new(device: &Arc<Device>, layout : &BindGroupLayout, size : u64) -> Self{
    let uniform_buffer = device.create_buffer(&BufferDescriptor {
        label: None,
        size,
        usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
        mapped_at_creation: false,
    });

    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            },
        ],
    });

    return Self{ bind_group, uniform_buffer};
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

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>){
        render_pass.set_bind_group(0, &self.bind_group, &[]);
    }
}
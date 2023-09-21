use std::sync::Arc;

use eframe::{
    egui_wgpu::RenderState,
    wgpu::{
        self, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
        BufferBindingType, Device, PrimitiveState, RenderPipeline, RenderPipelineDescriptor,
        ShaderModuleDescriptor, ShaderSource, Face, Features,
    },
};
use glam::Mat4;

use super::buffer::UniformBuffer;

pub struct Renderer {
    pipeline: RenderPipeline,
    buffer: UniformBuffer,
}

impl Renderer {
    pub fn new(device: &Arc<Device>, state: &RenderState) -> Self {
        let layout = get_layout(device, &[uniform()]);
        let pipeline = build_shader(
            device,
            state,
            "sphere renderer",
            include_str!("./../shaders/quad_sphere.wgsl"),
            &layout,
        );
        let buffer = UniformBuffer::new(device, &layout, 4 * 2 + 4 * 16 + 8);

        Self { pipeline, buffer }
    }

    pub fn prepare(&self, queue: &wgpu::Queue, projection: &Mat4) {
        self.buffer.write_mat(queue, 16, projection);
        self.buffer.write(queue, 0, &[2. as f32, 8. as f32]);
    }

    pub fn paint<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        self.buffer.bind(render_pass);
        //render_pass.draw(0..(8 * 8 * 4), 0..1);
        render_pass.draw(0..(8 * 8 * 6 * 4), 0..1);
    }
}

pub fn uniform() -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

pub fn get_layout(device: &Arc<Device>, buffers: &[BindGroupLayoutEntry]) -> BindGroupLayout {
    let bind_group_layout: BindGroupLayout =
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: buffers,
        });
    return bind_group_layout;
}

pub fn build_shader(
    device: &Arc<Device>,
    state: &RenderState,
    label: &str,
    source: &str,
    layout: &BindGroupLayout,
) -> RenderPipeline {
    let shader = device.create_shader_module(ShaderModuleDescriptor {
        label: Some(label),
        source: ShaderSource::Wgsl(source.into()),
    });

    let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some(label),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(label),
                bind_group_layouts: &[&layout],
                push_constant_ranges: &[],
            }),
        ),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vert_main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "frag_main",
            targets: &[Some(state.target_format.into())],
        }),
        primitive: PrimitiveState {
            topology: wgpu::PrimitiveTopology::LineList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Cw,
            cull_mode: None,
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    return pipeline;
}

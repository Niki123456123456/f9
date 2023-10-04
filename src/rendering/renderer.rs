use std::sync::Arc;

use eframe::{
    egui_wgpu::RenderState,
    wgpu::{
        self, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, Device,
        PrimitiveState, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor,
        ShaderSource, PrimitiveTopology,
    },
};

use crate::project::Project;

pub struct RenderShader {
    pipeline: RenderPipeline,
    get_draw_count: Box<dyn Fn(&Project) -> u32 + Send + Sync>,
}

impl RenderShader {
    pub fn new(
        device: &Arc<Device>,
        state: &RenderState,
        layout: &BindGroupLayout,
        label: &str,
        source: &str, 
        topology: PrimitiveTopology,
        get_draw_count: &'static (dyn Fn(&Project) -> u32 + Send + Sync),
    ) -> Self {
        Self {
            pipeline: build_shader(device, state, label, source, &layout, topology),
            get_draw_count: Box::new(get_draw_count),
        }
    }
}

pub struct Renderer {
    shaders: Vec<RenderShader>,
}

impl Renderer {
    pub fn new(device: &Arc<Device>, state: &RenderState) -> Self {
        let layout = get_layout(device, &[uniform(0), storage(1), storage(2), storage(3), storage(4)]);
        let shaders = vec![
            /*RenderShader::new(
                device,
                state,
                &layout,
                "quad_sphere",
                include_str!("./../shaders/quad_sphere.wgsl"),
                &|project| 8 * 8 * 6 * 4,
            ),
            RenderShader::new(
                device,
                state,
                &layout,
                "uv_sphere",
                include_str!("./../shaders/uv_sphere.wgsl"),
                &|project| 8 * 8 * 4,
            ),*/
            RenderShader::new(
                device,
                state,
                &layout,
                "axis",
                include_str!("./../shaders/axis.wgsl"),
                PrimitiveTopology::LineList,
                &|project| project.state.components.axises.array.len() as u32 * 4,
            ),
            RenderShader::new(
                device,
                state,
                &layout,
                "grid",
                include_str!("./../shaders/grid.wgsl"),
                PrimitiveTopology::LineList,
                &|project| project.state.components.axises.array.len() as u32 * 11 * 2 * 2,
            ),
            RenderShader::new(
                device,
                state,
                &layout,
                "arrow",
                include_str!("./../shaders/arrow.wgsl"),
                PrimitiveTopology::LineList,
                &|project| project.state.components.axises.array.len() as u32,
            ),
            RenderShader::new(
                device,
                state,
                &layout,
                "point",
                include_str!("./../shaders/point.wgsl"),
                PrimitiveTopology::TriangleList,
                &|project| project.state.components.axises.array.len() as u32 * 6,
            ),
        ];

        Self { shaders }
    }

    pub fn paint<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, project: &'a Project) {
        render_pass.set_bind_group(0, &project.state.uniform_buffer.bind_group, &[]);
        for shader in self.shaders.iter() {
            render_pass.set_pipeline(&shader.pipeline);
            let draw_count = (shader.get_draw_count)(&project);
            render_pass.draw(0..draw_count, 0..1);
        }
    }
}

pub fn uniform(index: u32) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding: index,
        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

pub fn storage(index: u32) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding: index,
        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only: true },
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
    layout: &BindGroupLayout, topology: wgpu::PrimitiveTopology
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
            topology,
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

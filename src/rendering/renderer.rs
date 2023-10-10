use std::sync::Arc;

use eframe::{
    egui_wgpu::RenderState,
    wgpu::{
        self, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, ComputePipeline,
        DepthStencilState, Device, PrimitiveState, PrimitiveTopology, RenderPipeline,
        RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, TextureFormat,
    },
};

use crate::project::Project;

pub struct ComputeShader {
    pipeline: ComputePipeline,
    get_draw_count: Box<dyn Fn(&Project) -> (u32, u32, u32) + Send + Sync>,
}

impl ComputeShader {
    pub fn new(
        device: &Arc<Device>,
        layout: &BindGroupLayout,
        label: &str,
        source: &str,
        get_draw_count: &'static (dyn Fn(&Project) -> (u32, u32, u32) + Send + Sync),
    ) -> Self {
        Self {
            pipeline: build_compute_shader(
                device,
                label,
                &(include_str!("./../shaders/common.wgsl").to_owned() + source),
                layout,
            ),
            get_draw_count: Box::new(get_draw_count),
        }
    }
}

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
            pipeline: build_shader(
                device,
                state,
                label,
                &(include_str!("./../shaders/common.wgsl").to_owned() + source),
                layout,
                topology,
            ),
            get_draw_count: Box::new(get_draw_count),
        }
    }
}

pub struct Renderer {
    shaders: Vec<RenderShader>,
    compute_shaders: Vec<ComputeShader>,
}

impl Renderer {
    pub fn new(device: &Arc<Device>, state: &RenderState) -> Self {
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
                //storage(9),
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
                //storage_writeable(9),
            ],
        );
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
                "point",
                include_str!("./../shaders/point.wgsl"),
                PrimitiveTopology::TriangleList,
                &|project| project.state.components.axises.array.len() as u32 * 6,
            ),
            RenderShader::new(
                device,
                state,
                &layout,
                "line",
                include_str!("./../shaders/line.wgsl"),
                PrimitiveTopology::LineList,
                &|project| project.state.components.axises.array.len() as u32 * 2,
            ),
            RenderShader::new(
                device,
                state,
                &layout,
                "bezier",
                include_str!("./../shaders/bezier.wgsl"),
                PrimitiveTopology::LineList,
                &|project| project.state.components.beziers.array.len() as u32 * 2 * 51,
            ),
            RenderShader::new(
                device,
                state,
                &layout,
                "circle",
                include_str!("./../shaders/circle.wgsl"),
                PrimitiveTopology::LineList,
                &|project| project.state.components.beziers.array.len() as u32 * 2 * 51,
            ),
            RenderShader::new(
                device,
                state,
                &layout,
                "arrow",
                include_str!("./../shaders/arrow.wgsl"),
                PrimitiveTopology::LineList,
                &|project| project.state.components.axises.array.len() as u32 * 6,
            ),
            RenderShader::new(
                device,
                state,
                &layout,
                "arrow_plane",
                include_str!("./../shaders/arrow_plane.wgsl"),
                PrimitiveTopology::TriangleList,
                &|project| project.state.components.axises.array.len() as u32 * 6,
            ),
        ];

        let compute_shaders = vec![
            ComputeShader::new(
                device,
                &compute_layout,
                "point_com",
                include_str!("./../shaders/point_com.wgsl"),
                &|project| (project.state.components.points.array.len() as u32, 1, 1),
            ),
            ComputeShader::new(
                device,
                &compute_layout,
                "line_com",
                include_str!("./../shaders/line_com.wgsl"),
                &|project| (project.state.components.lines.array.len() as u32, 1, 1),
            ),
            ComputeShader::new(
                device,
                &compute_layout,
                "bezier_com",
                include_str!("./../shaders/bezier_com.wgsl"),
                &|project| (project.state.components.beziers.array.len() as u32, 1, 1),
            ),
            ComputeShader::new(
                device,
                &compute_layout,
                "circle_com",
                include_str!("./../shaders/circle_com.wgsl"),
                &|project| (project.state.components.circles.array.len() as u32, 1, 1),
            ),
            ComputeShader::new(
                device,
                &compute_layout,
                "arrow_com",
                include_str!("./../shaders/arrow_com.wgsl"),
                &|project| (project.state.components.arrows.array.len() as u32, 1, 1),
            ),
            ComputeShader::new(
                device,
                &compute_layout,
                "arrow_plane_com",
                include_str!("./../shaders/arrow_plane_com.wgsl"),
                &|project| (project.state.components.arrows.array.len() as u32, 1, 1),
            ),
        ];

        Self {
            shaders,
            compute_shaders,
        }
    }

    pub fn compute<'a>(&'a self, mut pass: wgpu::ComputePass<'a>, project: &'a Project) {
        pass.set_bind_group(0, &project.state.uniform_buffer.compute_bind_group, &[]);
        pass.set_bind_group(1, &project.state.uniform_buffer.atomic_bind_group, &[]);
        for shader in self.compute_shaders.iter() {
            pass.set_pipeline(&shader.pipeline);
            let draw_count = (shader.get_draw_count)(&project);
            pass.dispatch_workgroups(draw_count.0, draw_count.1, draw_count.2);
        }
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
        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT | wgpu::ShaderStages::COMPUTE,
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

pub fn storage_writeable(index: u32) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding: index,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only: false },
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

pub fn build_compute_shader(
    device: &Arc<Device>,
    label: &str,
    source: &str,
    layout: &BindGroupLayout,
) -> ComputePipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(source)).into(),
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some(label),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(label),
                bind_group_layouts: &[&layout, &get_layout(device, &[storage_writeable(0)])],
                push_constant_ranges: &[],
            }),
        ),
        module: &shader,
        entry_point: "main",
    });
    return pipeline;
}

pub fn build_shader(
    device: &Arc<Device>,
    state: &RenderState,
    label: &str,
    source: &str,
    layout: &BindGroupLayout,
    topology: wgpu::PrimitiveTopology,
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
            targets: //&[Some(state.target_format.into())],
            &[
                Some(wgpu::ColorTargetState{
                format: state.target_format,

                blend: Some(wgpu::BlendState{
                    color: wgpu::BlendComponent{
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,},
                    alpha: wgpu::BlendComponent::OVER
                }),

                write_mask: wgpu::ColorWrites::ALL,
            })]
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
        depth_stencil: Some(DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    return pipeline;
}

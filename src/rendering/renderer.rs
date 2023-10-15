use std::sync::Arc;

use eframe::{
    egui_wgpu::RenderState,
    wgpu::{
        self, BindGroupDescriptor, BindGroupLayout, BindGroupLayoutDescriptor,
        BindGroupLayoutEntry, ComputePipeline, DepthStencilState, Device, PrimitiveState,
        PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor,
        ShaderSource, TextureFormat,
    },
};

use crate::{
    component_collection::ComponentCollection,
    components,
    project::{self, Project},
};

pub struct ComputeShader<T> {
    // RenderPipeline, ComputePipeline
    pub pipeline: T,
    pub get_draw_count: Box<dyn Fn(&Project) -> (u32, u32, u32) + Send + Sync>,
    pub storage_count: u32,
    pub get_buffers: Box<dyn Fn(&ComponentCollection) -> Vec<&wgpu::Buffer> + Send + Sync>,
    pub get_bindgroup: Box<dyn Fn(&Device, u32) -> BindGroupLayout + Send + Sync>,
    pub label: &'static str,
}

impl<T> ComputeShader<T> {
    pub fn get_bindgroup(
        &self,
        device: &Arc<Device>,
        components: &ComponentCollection,
    ) -> wgpu::BindGroup {
        let buffers: Vec<_> = (self.get_buffers)(components)
            .iter()
            .enumerate()
            .map(|(i, buffer)| wgpu::BindGroupEntry {
                binding: i as u32,
                resource: buffer.as_entire_binding(),
            })
            .collect();

        return device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &(self.get_bindgroup)(&device, self.storage_count),
            entries: &buffers,
        });
    }
}

pub fn new_compute_shader(
    device: &Arc<Device>,
    label: &'static str,
    source: &str,
    get_draw_count: &'static (dyn Fn(&Project) -> (u32, u32, u32) + Send + Sync),
    get_buffers: &'static (dyn Fn(&ComponentCollection) -> Vec<&wgpu::Buffer> + Send + Sync),
    storage_count: u32,
) -> ComputeShader<ComputePipeline> {
    ComputeShader {
        pipeline: build_compute_shader(
            device,
            label,
            &(include_str!("./../shaders/common.wgsl").to_owned() + source),
            &get_buffer_layout(
                device,
                storage_count,
                wgpu::ShaderStages::COMPUTE,
                wgpu::BufferBindingType::Storage { read_only: false },
            ),
        ),
        get_draw_count: Box::new(get_draw_count),
        get_buffers: Box::new(get_buffers),
        get_bindgroup: Box::new(|device, storage_count| {
            get_buffer_layout(
                device,
                storage_count,
                wgpu::ShaderStages::COMPUTE,
                wgpu::BufferBindingType::Storage { read_only: false },
            )
        }),
        storage_count,
        label,
    }
}

pub fn new_shader(
    device: &Arc<Device>,
    state: &RenderState,
    label: &'static str,
    source: &str,
    topology: PrimitiveTopology,
    get_draw_count: &'static (dyn Fn(&Project) -> u32 + Send + Sync),
    get_buffers: &'static (dyn Fn(&ComponentCollection) -> Vec<&wgpu::Buffer> + Send + Sync),
    storage_count: u32,
) -> ComputeShader<RenderPipeline> {
    ComputeShader {
        pipeline: build_shader(
            device,
            state,
            label,
            &(include_str!("./../shaders/common.wgsl").to_owned() + source),
            &get_buffer_layout(
                device,
                storage_count,
                wgpu::ShaderStages::VERTEX_FRAGMENT,
                wgpu::BufferBindingType::Storage { read_only: true },
            ),
            topology,
        ),
        get_draw_count: Box::new(|p| {
            let count = (get_draw_count)(p);
            return (count, 1, 1);
        }),
        get_buffers: Box::new(get_buffers),
        get_bindgroup: Box::new(|device, storage_count| {
            get_buffer_layout(
                device,
                storage_count,
                wgpu::ShaderStages::VERTEX_FRAGMENT,
                wgpu::BufferBindingType::Storage { read_only: true },
            )
        }),
        storage_count,
        label,
    }
}

pub struct Renderer {
    pub shaders: Vec<ComputeShader<RenderPipeline>>,
    pub compute_shaders: Vec<ComputeShader<ComputePipeline>>,
}

impl Renderer {
    pub fn new(device: &Arc<Device>, state: &RenderState) -> Self {
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
            new_shader(
                device,
                state,
                "axis",
                include_str!("./../shaders/axis.wgsl"),
                PrimitiveTopology::LineList,
                &|project| project.state.components.axises.array.len() as u32 * 4,
                &|components| vec![&components.axises.buffer],
                1,
            ),
            new_shader(
                device,
                state,
                "grid",
                include_str!("./../shaders/grid.wgsl"),
                PrimitiveTopology::LineList,
                &|project| project.state.components.grids.array.len() as u32 * 11 * 2 * 2,
                &|components| vec![&components.grids.buffer],
                1,
            ),
            new_shader(
                device,
                state,
                "point",
                include_str!("./../shaders/point.wgsl"),
                PrimitiveTopology::TriangleList,
                &|project| project.state.components.points.array.len() as u32 * 6,
                &|components| vec![&components.points.buffer],
                1,
            ),
            new_shader(
                device,
                state,
                "line",
                include_str!("./../shaders/line.wgsl"),
                PrimitiveTopology::LineList,
                &|project| project.state.components.lines.array.len() as u32 * 2,
                &|components| vec![&components.points.buffer, &components.lines.buffer],
                2,
            ),
            new_shader(
                device,
                state,
                "bezier",
                include_str!("./../shaders/bezier.wgsl"),
                PrimitiveTopology::LineList,
                &|project| project.state.components.beziers.array.len() as u32 * 2 * 51,
                &|components| vec![&components.points.buffer, &components.beziers.buffer],
                2,
            ),
            new_shader(
                device,
                state,
                "circle",
                include_str!("./../shaders/circle.wgsl"),
                PrimitiveTopology::LineList,
                &|project| project.state.components.circles.array.len() as u32 * 2 * 51,
                &|components| vec![&components.points.buffer, &components.circles.buffer],
                2,
            ),
            new_shader(
                device,
                state,
                "arrow",
                include_str!("./../shaders/arrow.wgsl"),
                PrimitiveTopology::LineList,
                &|project| project.state.components.arrows.array.len() as u32 * 6,
                &|components| vec![&components.arrows.buffer],
                1,
            ),
            new_shader(
                device,
                state,
                "arrow_plane",
                include_str!("./../shaders/arrow_plane.wgsl"),
                PrimitiveTopology::TriangleList,
                &|project| project.state.components.arrow_planes.array.len() as u32 * 6,
                &|components| vec![&components.arrow_planes.buffer],
                1,
            ),
        ];

        let compute_shaders = vec![
            new_compute_shader(
                device,
                "point_com",
                include_str!("./../shaders/point_com.wgsl"),
                &|project| (project.state.components.points.array.len() as u32, 1, 1),
                &|components| vec![&components.points.buffer],
                1,
            ),
            new_compute_shader(
                device,
                "line_com",
                include_str!("./../shaders/line_com.wgsl"),
                &|project| (project.state.components.lines.array.len() as u32, 1, 1),
                &|components| vec![&components.points.buffer, &components.lines.buffer],
                2,
            ),
            new_compute_shader(
                device,
                "bezier_com",
                include_str!("./../shaders/bezier_com.wgsl"),
                &|project| (project.state.components.beziers.array.len() as u32, 1, 1),
                &|components| vec![&components.points.buffer, &components.beziers.buffer],
                2,
            ),
            new_compute_shader(
                device,
                "circle_com",
                include_str!("./../shaders/circle_com.wgsl"),
                &|project| (project.state.components.circles.array.len() as u32, 1, 1),
                &|components| vec![&components.points.buffer, &components.circles.buffer],
                2,
            ),
            new_compute_shader(
                device,
                "arrow_com",
                include_str!("./../shaders/arrow_com.wgsl"),
                &|project| (project.state.components.arrows.array.len() as u32, 1, 1),
                &|components| vec![&components.arrows.buffer],
                1,
            ),
            new_compute_shader(
                device,
                "arrow_plane_com",
                include_str!("./../shaders/arrow_plane_com.wgsl"),
                &|project| (project.state.components.arrow_planes.array.len() as u32, 1, 1),
                &|components| vec![&components.arrow_planes.buffer],
                1,
            ),
        ];

        Self {
            shaders,
            compute_shaders,
        }
    }

    pub fn compute<'a>(&'a self, mut pass: wgpu::ComputePass<'a>, project: &'a Project) {
        pass.set_bind_group(0, &project.state.uniform_buffer.uniform_bind_group, &[]);
        pass.set_bind_group(1, &project.state.uniform_buffer.hover_bind_group, &[]);
        for shader in self.compute_shaders.iter() {
            pass.set_bind_group(
                2,
                &project
                    .state
                    .uniform_buffer
                    .bind_groups
                    .get(shader.label)
                    .unwrap(),
                &[],
            );
            pass.set_pipeline(&shader.pipeline);
            let draw_count = (shader.get_draw_count)(&project);
            pass.dispatch_workgroups(draw_count.0, draw_count.1, draw_count.2);
        }
    }

    pub fn paint<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>, project: &'a Project) {
        pass.set_bind_group(0, &project.state.uniform_buffer.uniform_bind_group, &[]);
        for shader in self.shaders.iter() {
            pass.set_bind_group(
                1,
                &project
                    .state
                    .uniform_buffer
                    .bind_groups
                    .get(shader.label)
                    .unwrap(),
                &[],
            );
            pass.set_pipeline(&shader.pipeline);
            let draw_count = (shader.get_draw_count)(&project);
            pass.draw(0..draw_count.0, 0..1);
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
    return device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: None,
        entries: buffers,
    });
}

pub fn get_buffer_layout(
    device: &Device,
    count: u32,
    visibility: wgpu::ShaderStages,
    ty: wgpu::BufferBindingType,
) -> BindGroupLayout {
    let v: Vec<_> = (0..count)
        .into_iter()
        .map(|index| BindGroupLayoutEntry {
            binding: index,
            visibility,
            ty: wgpu::BindingType::Buffer {
                ty,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        })
        .collect();
    return device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: None,
        entries: &v,
    });
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
                bind_group_layouts: &[
                    &get_layout(device, &[uniform(0)]),
                    &get_layout(device, &[storage_writeable(0), storage_writeable(1)]),
                    &layout,
                ],
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
                bind_group_layouts: &[&get_layout(device, &[uniform(0)]), &layout],
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

use eframe::{
    egui_wgpu::wgpu::util::DeviceExt,
    egui_wgpu::{self, wgpu, RenderState},
    wgpu::{AdapterInfo, BindGroupLayout, BufferDescriptor, BufferUsages, Device},
};
use egui::{epaint::Shadow, Color32, Margin, Rounding, Stroke, Vec2};
use instant::{Duration, Instant};
use std::{num::NonZeroU64, sync::Arc};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    #[serde(skip)]
    adapter_info: Option<AdapterInfo>,
    #[serde(skip)]
    last_render_time: Instant,
    #[serde(skip)]
    cp: Option<ComputeResources>,
    #[serde(skip)]
    raster: Option<RasterResources>,
    #[serde(skip)]
    screen: Option<TriangleRenderResources>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            adapter_info: None,
            last_render_time: Instant::now(),
            cp: None,
            raster: None,
            screen: None,
        }
    }
}

fn build_screen_pass(
    device: &Arc<Device>,
    wgpu_render_state: &RenderState,
    outputcolor_buffer: &wgpu::Buffer,
) -> TriangleRenderResources2 {
    let label = Some("screen_pass");
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label,
        source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/fullscreenQuad.wgsl").into()),
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label,
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label,
                bind_group_layouts: &[&bind_group_layout],
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
            targets: &[Some(wgpu_render_state.target_format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    let uniform_buffersize = 4 * 2;
    let uniform_buffer = device.create_buffer(&BufferDescriptor {
        label,
        size: uniform_buffersize,
        usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
        mapped_at_creation: false,
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label,
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: outputcolor_buffer.as_entire_binding(),
            },
        ],
    });

    return TriangleRenderResources2 {
        pipeline,
        bind_group,
        uniform_buffer,
    };
}

fn build_raster_pass(device: &Arc<Device>, wgpu_render_state: &RenderState) -> RasterResources {
    let label = Some("raster_pass");

    let vertex_buffer = device.create_buffer(&BufferDescriptor {
        label,
        size: 4 * 6,
        usage: BufferUsages::STORAGE,
        mapped_at_creation: false,
    });

    let width = 3000;
    let height = 3000;
    let color_channels = 3;
    let outputcolor_buffersize = 4 * (width * height) * color_channels;
    let outputcolor_buffer = device.create_buffer(&BufferDescriptor {
        label,
        size: outputcolor_buffersize,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let uniform_buffersize = 4 * 2 + 4 * 16 + 8;
    let uniform_buffer = device.create_buffer(&BufferDescriptor {
        label,
        size: uniform_buffersize,
        usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
        mapped_at_creation: false,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label,
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: outputcolor_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: vertex_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: uniform_buffer.as_entire_binding(),
            },
        ],
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label,
        source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/computeRasterizer.wgsl").into()),
    });

    let raster_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label,
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            }),
        ),
        module: &shader,
        entry_point: "main",
    });

    let clear_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label,
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            }),
        ),
        module: &shader,
        entry_point: "clear",
    });

    return RasterResources {
        clear_pipeline,
        raster_pipeline,
        bind_group,
        uniform_buffer,
        vertex_buffer,
        outputcolor_buffer,
    };
}

fn build_compute_shader(device: &Arc<Device>, wgpu_render_state: &RenderState) -> ComputeResources {
    let label = Some("compute");
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label,
        source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/compute.wgsl").into()),
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label,
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label,
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label,
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: "main",
    });

    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label,
        contents: bytemuck::cast_slice(&[0.0_f32; 4]), // 16 bytes aligned!
        // Mapping at creation (as done by the create_buffer_init utility) doesn't require us to to add the MAP_WRITE usage
        // (this *happens* to workaround this bug )
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label,
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
    });

    return ComputeResources {
        pipeline,
        bind_group,
        buffer,
    };
}

fn build_test_shader(
    device: &Arc<Device>,
    wgpu_render_state: &RenderState,
) -> TriangleRenderResources {
    let label = Some("custom3d");
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label,
        source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/shader.wgsl").into()),
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label,
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: NonZeroU64::new(16),
            },
            count: None,
        }],
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label,
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            }),
        ),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu_render_state.target_format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label,
        contents: bytemuck::cast_slice(&[0.0_f32; 4]), // 16 bytes aligned!
        // Mapping at creation (as done by the create_buffer_init utility) doesn't require us to to add the MAP_WRITE usage
        // (this *happens* to workaround this bug )
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label,
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: uniform_buffer.as_entire_binding(),
        }],
    });

    return TriangleRenderResources {
        pipeline,
        bind_group,
        uniform_buffer,
    };
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let wgpu_render_state = cc.wgpu_render_state.as_ref().unwrap();
        let device = &wgpu_render_state.device;

        wgpu_render_state
            .renderer
            .write()
            .paint_callback_resources
            .insert(build_test_shader(device, wgpu_render_state));

            wgpu_render_state
            .renderer
            .write()
            .paint_callback_resources
            .insert(WindowSize{ width: 1000.0, height: 1000.0 });

        let mut app: TemplateApp = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        let raster = build_raster_pass(device, wgpu_render_state);
        let screen_pass = build_screen_pass(device, wgpu_render_state, &raster.outputcolor_buffer);

        wgpu_render_state
            .renderer
            .write()
            .paint_callback_resources
            .insert(screen_pass);

           
        app.adapter_info = Some(wgpu_render_state.adapter.get_info());
        app.cp = Some(build_compute_shader(device, wgpu_render_state));
        app.raster = Some(raster);
        //app.screen = Some(screen_pass);
        return app;
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::CentralPanel::default()
            .frame(egui::Frame {
                inner_margin: Margin::same(0.),
                outer_margin: Margin::same(0.),
                rounding: Rounding::none(),
                shadow: Shadow::NONE,
                fill: Color32::TRANSPARENT,
                stroke: Stroke::NONE,
            })
            .show(ctx, |ui| {
                let mut spacing = ui.spacing_mut();
                spacing.item_spacing = Vec2::ZERO;

                let (rect, response) =
                    ui.allocate_at_least(ui.available_size(), egui::Sense::drag());
                let renderstate = _frame.wgpu_render_state().unwrap();
                {
                    let mut writer = renderstate.renderer.write();
                    let s : &mut WindowSize = writer.paint_callback_resources.get_mut().unwrap();
                    if s.width != rect.width() || s.height != rect.height() {
                        s.width = rect.width();
                        s.height = rect.height();
                        println!("{} {}", s.width, s.height);
                    }
                }
                if let Some(cp) = &self.raster {
                    cp.execute(renderstate, rect.size());
                }
                

                
                let cb = egui_wgpu::CallbackFn::new()
                    .prepare(move |device, queue, _encoder, paint_callback_resources| {
                        let resources: &TriangleRenderResources2 =
                            paint_callback_resources.get().unwrap();
                            let size: &WindowSize =
                            paint_callback_resources.get().unwrap();
                        resources.set_size(device, queue, size.width, size.height);
                        Vec::new()
                    })
                    .paint(move |_info, render_pass, paint_callback_resources| {
                        let resources: &TriangleRenderResources2 =
                            paint_callback_resources.get().unwrap();
                        resources.paint2(render_pass);
                    });

                let callback = egui::PaintCallback {
                    rect,
                    callback: Arc::new(cb),
                };

                ui.painter().add(callback);
            });

        if let Some(adapter_info) = &self.adapter_info {
            egui::Window::new("GPU Info").show(&ctx, |ui| {
                ui.label(format!("backend: {:?}", adapter_info.backend));
                ui.label(format!("name: {}", adapter_info.name));
                ui.label(format!("device: {}", adapter_info.device));
                ui.label(format!("device_type: {:?}", adapter_info.device_type));
                ui.label(format!("driver: {}", adapter_info.driver));
                ui.label(format!("driver_info: {}", adapter_info.driver_info));
                let ele = self.last_render_time.elapsed();
                let fps = 1.0 / ele.as_secs_f64();
                ui.label(format!("duration {:.0}ms", ele.as_millis() as f64));
                ui.label(format!("frames {:.0}/s", fps));
                self.last_render_time = Instant::now();
            });
        }

        ctx.request_repaint();
    }
}

struct WindowSize{
    width : f32,
    height: f32,
}

struct RasterResources {
    clear_pipeline: wgpu::ComputePipeline,
    raster_pipeline: wgpu::ComputePipeline,
    bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,
    outputcolor_buffer: wgpu::Buffer,
}

struct ComputeResources {
    pipeline: wgpu::ComputePipeline,
    bind_group: wgpu::BindGroup,
    buffer: wgpu::Buffer,
}

impl RasterResources {
    fn execute(&self, renderstate: &RenderState, size : Vec2) {
        renderstate.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[size.x, size.y]),
        );


        let mut encoder =
            renderstate
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("clear Encoder"),
                });

        {
            let mut passencoder: wgpu::ComputePass<'_> =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Some("clear pass"),
                });
            passencoder.set_pipeline(&self.clear_pipeline);
            passencoder.set_bind_group(0, &self.bind_group, &[]);
            let work_group_count = ((size.x as f32) * (size.y as f32) / (256 as f32)).ceil() as u32;
            passencoder.dispatch_workgroups(work_group_count, 1, 1);
        }
        renderstate.queue.submit(Some(encoder.finish()));

        let mut encoder =
            renderstate
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Raster Encoder"),
                });
        {
            let mut passencoder = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("raster pass"),
            });
            passencoder.set_pipeline(&self.raster_pipeline);
            passencoder.set_bind_group(0, &self.bind_group, &[]);
            passencoder.dispatch_workgroups(1, 1, 1);
        }
        renderstate.queue.submit(Some(encoder.finish()));
    }
}

struct TriangleRenderResources {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
}
struct TriangleRenderResources2 {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
}
impl TriangleRenderResources2 {
    fn set_size(&self, _device: &wgpu::Device, queue: &wgpu::Queue, width: f32, height: f32) {
        // Update our uniform buffer with the angle from the UI
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[width, height]),
        );
    }

    fn paint2<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }
}

impl TriangleRenderResources {
    fn prepare(&self, _device: &wgpu::Device, queue: &wgpu::Queue, angle: f32) {
        // Update our uniform buffer with the angle from the UI
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[angle, 0.0, 0.0, 0.0]),
        );
    }

    fn paint<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }
}

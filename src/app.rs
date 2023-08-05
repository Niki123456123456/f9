use eframe::{
    egui_wgpu::wgpu::util::DeviceExt,
    egui_wgpu::{self, wgpu, RenderState},
    wgpu::{AdapterInfo, BindGroupLayout, Buffer, BufferDescriptor, BufferUsages, Device, Queue},
};
use egui::{epaint::Shadow, vec2, Color32, Margin, Pos2, Rect, Rounding, Stroke, Vec2};
use glam::{Mat4, Vec3};
use instant::{Duration, Instant};
use std::{num::NonZeroU64, sync::Arc};

use crate::{camera::Camera, rendering::sphere::Renderer};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    #[serde(skip)]
    adapter_info: Option<AdapterInfo>,
    #[serde(skip)]
    last_render_time: Instant,
    #[serde(skip)]
    raster: Option<RasterResources>,
    #[serde(skip)]
    camera: Camera,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            adapter_info: None,
            last_render_time: Instant::now(),
            raster: None,
            camera: Camera::default(),
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
        size: 4 * 36,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
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

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let wgpu_render_state = cc.wgpu_render_state.as_ref().unwrap();

        let device = &wgpu_render_state.device;

        wgpu_render_state
            .renderer
            .write()
            .paint_callback_resources
            .insert(WindowSize {
                width: 1000.0,
                height: 1000.0,
            });

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

            wgpu_render_state
            .renderer
            .write()
            .paint_callback_resources
            .insert(crate::rendering::sphere::Renderer::new(device, wgpu_render_state));

        app.adapter_info = Some(wgpu_render_state.adapter.get_info());
        app.raster = Some(raster);
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

                self.camera.viewport = Rect {
                    min: Pos2 {
                        x: rect.min.x * ctx.pixels_per_point(),
                        y: rect.min.y * ctx.pixels_per_point(),
                    },
                    max: Pos2 {
                        x: rect.max.x * ctx.pixels_per_point(),
                        y: rect.max.y * ctx.pixels_per_point(),
                    },
                };

                self.camera.update_input(ctx);
                self.camera.calculate_matrixs();

                let pos = ctx.input(|e| e.pointer.hover_pos()).unwrap_or(Pos2::ZERO);
                self.camera.update_ray(vec2(
                    pos.x * ctx.pixels_per_point(),
                    pos.y * ctx.pixels_per_point(),
                ));

                let renderstate = _frame.wgpu_render_state().unwrap();
                {
                    let mut writer = renderstate.renderer.write();
                    let s: &mut WindowSize = writer.paint_callback_resources.get_mut().unwrap();
                    if s.width != rect.width() || s.height != rect.height() {
                        s.width = rect.width();
                        s.height = rect.height();
                        println!("{} {}", s.width, s.height);
                    }
                }
                if let Some(cp) = &self.raster {
                    cp.execute(
                        renderstate,
                        rect.size(),
                        &self.camera.projection_view_matrix,
                    );
                }

                let cb = egui_wgpu::CallbackFn::new()
                    .prepare(move |device, queue, _encoder, paint_callback_resources| {
                        let resources: &TriangleRenderResources2 =
                            paint_callback_resources.get().unwrap();
                        let size: &WindowSize = paint_callback_resources.get().unwrap();
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

                //ui.painter().add(callback);

                {let reader = renderstate.renderer.read();
                let resources: &Renderer = reader.paint_callback_resources.get().unwrap();
                        resources.prepare(&renderstate.queue, &self.camera.projection_view_matrix);}

                let cb = egui_wgpu::CallbackFn::new()
                    .prepare(move |device, queue, _encoder, paint_callback_resources| {
                    
                        Vec::new()
                    })
                    .paint(move |_info, render_pass, paint_callback_resources| {
                        let resources: &Renderer =
                            paint_callback_resources.get().unwrap();
                        resources.paint(render_pass);
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

#[derive(Clone)]
#[repr(C)]
pub struct Vertex {
    pub position: Vec3,
    pub direction: Vec3,
}

struct WindowSize {
    width: f32,
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

impl RasterResources {
    fn execute(&self, renderstate: &RenderState, size: Vec2, projection: &Mat4) {
        let mx_ref: &[f32; 16] = projection.as_ref();

        renderstate
            .queue
            .write_buffer(&self.uniform_buffer, 16, bytemuck::cast_slice(mx_ref));
        renderstate.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[size.x, size.y]),
        );
        let array = vec![
            Vertex {
                position: Vec3::ZERO,
                direction: Vec3::X,
            },
            Vertex {
                position: Vec3::ZERO,
                direction: Vec3::Y,
            },
            Vertex {
                position: Vec3::ZERO,
                direction: Vec3::Z,
            },
        ];

        write_buffer(&array, &renderstate.queue, &self.vertex_buffer);

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
            passencoder.dispatch_workgroups(array.len() as u32, 1000, 1);
        }
        renderstate.queue.submit(Some(encoder.finish()));
    }
}

fn write_buffer<T>(array: &[T], queue: &Queue, buffer: &Buffer) {
    unsafe {
        let single_size = std::mem::size_of::<Vertex>() * array.len();
        let data = std::slice::from_raw_parts(array as *const [T] as *const u8, single_size);
        queue.write_buffer(buffer, 0, data);
    }
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

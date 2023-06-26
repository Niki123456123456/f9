use eframe::{
    egui_wgpu::wgpu::util::DeviceExt,
    egui_wgpu::{self, wgpu, RenderState},
    wgpu::{AdapterInfo, Device},
};
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
    cp : Option<ComputeResources>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            adapter_info: None,
            last_render_time: Instant::now(),
            cp: None,
        }
    }
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

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label,
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    //device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor { label: (), layout: (), module: (), entry_point: () })

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label,
        layout: Some(&pipeline_layout),
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

        

        let mut app: TemplateApp = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        app.adapter_info = Some(wgpu_render_state.adapter.get_info());
        app.cp= Some(build_compute_shader(device, wgpu_render_state));
        return app;
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(cp) = & self.cp {
            let wstate = _frame.wgpu_render_state().unwrap();
           
            let mut encoder = wstate.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Compute Encoder"),
            });
            
            {
                let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Some("Compute Pass"),
                });
                compute_pass.set_pipeline(&cp.pipeline);
                compute_pass.set_bind_group(0, &cp.bind_group, &[]);
                compute_pass.dispatch_workgroups(4, 1, 1);
            
            }
            
            let command_buffer = encoder.finish();
            let i = wstate.queue.submit(Some(command_buffer));
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("f9");

            let (rect, response) =
                ui.allocate_exact_size(egui::Vec2::splat(300.0), egui::Sense::drag());

            
            

            let cb = egui_wgpu::CallbackFn::new()
                .prepare(move |device, queue, _encoder, paint_callback_resources| {
                    let resources: &TriangleRenderResources =
                        paint_callback_resources.get().unwrap();
                    resources.prepare(device, queue, 0.0);
                    Vec::new()
                })
                .paint(move |_info, render_pass, paint_callback_resources| {
                    let resources: &TriangleRenderResources =
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
struct ComputeResources {
    pipeline: wgpu::ComputePipeline,
    bind_group: wgpu::BindGroup,
    buffer: wgpu::Buffer,
}

struct TriangleRenderResources {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
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

    fn paint<'rp>(&'rp self, render_pass: &mut wgpu::RenderPass<'rp>) {
        // Draw our triangle!
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }
}

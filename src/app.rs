use eframe::{
    egui_wgpu::wgpu::util::DeviceExt,
    egui_wgpu::{self, wgpu, CallbackResources, CallbackTrait, RenderState},
    wgpu::{
        AdapterInfo, BindGroupLayout, Buffer, BufferDescriptor, BufferUsages, Device, Queue,
        RenderPassColorAttachment, RenderPassDescriptor,
    },
};
use egui::{epaint::Shadow, vec2, Color32, Margin, Pos2, Rect, Response, Rounding, Stroke, Vec2};
use glam::{Mat4, Vec3};
use instant::{Duration, Instant};
use std::ops::DerefMut;
use std::{
    num::NonZeroU64,
    ops::Sub,
    sync::{Arc, Mutex},
};

use crate::{
    camera::{self, Camera},
    project::{self, Project, ProjectState},
    rendering::renderer::{self, Renderer},
    ui::{tabcontrol, main_menu::draw_commands}, commands::command::{Command, get_commands},
};

pub struct AppState {
    pub projects: Vec<Project>,
    pub selected_project: usize,
    pub renderer: Renderer,
    
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    #[serde(skip)]
    adapter_info: Option<AdapterInfo>,
    #[serde(skip)]
    last_render_time: Instant,
    #[serde(skip)]
    raster: Option<RasterResources>,
    #[serde(skip)]
    pub selected_project: usize,
    #[serde(skip)]
    pub commands: Vec<Command>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            adapter_info: None,
            last_render_time: Instant::now(),
            raster: None,
            selected_project: 0,
            commands: get_commands(),
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let wgpu_render_state = cc.wgpu_render_state.as_ref().unwrap();
        let device = &wgpu_render_state.device;

        let mut app: App = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        wgpu_render_state
            .renderer
            .write()
            .callback_resources
            .insert(AppState {
                projects: vec![Project::new(device, &wgpu_render_state.queue)],
                selected_project: 0,
                renderer: crate::rendering::renderer::Renderer::new(device, wgpu_render_state),
            });

        app.adapter_info = Some(wgpu_render_state.adapter.get_info());
        return app;
    }
}

struct RenderCallback;

impl CallbackTrait for RenderCallback {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _egui_encoder: &mut wgpu::CommandEncoder,
        _callback_resources: &mut CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        return Vec::new();
    }

    fn paint<'a>(
        &'a self,
        info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'a>,
        callback_resources: &'a egui_wgpu::CallbackResources,
    ) {
        let appstate: &AppState = callback_resources.get().unwrap();
        let project = &appstate.projects[appstate.selected_project];
        appstate.renderer.paint(render_pass, project);
    }
}

impl eframe::App for App {
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
                let spacing = ui.spacing_mut();
                spacing.item_spacing = Vec2::ZERO;

                let renderstate = _frame.wgpu_render_state().unwrap();

                tabcontrolheader(renderstate, ui);

                {
                    let mut writer = renderstate.renderer.write();
                    let appstate: &mut AppState = writer.callback_resources.get_mut().unwrap();
                    let project = &mut appstate.projects[appstate.selected_project];

                    draw_commands(ui, project, &self.commands);
                }

                let (rect, response) =
                    ui.allocate_at_least(ui.available_size(), egui::Sense::drag());

                {
                    let mut writer = renderstate.renderer.write();
                    let appstate: &mut AppState = writer.callback_resources.get_mut().unwrap();
                    let project = &mut appstate.projects[appstate.selected_project];

                    update_camera(&mut project.state, rect, response, ctx);
                    flush_buffer(project, renderstate, rect, ctx);
                }
                run_compute_pass(renderstate);

                

                run_render_pass(ui, rect);
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

fn run_render_pass(ui: &mut egui::Ui, rect: Rect) {
    ui.painter().add(egui_wgpu::Callback::new_paint_callback(
        rect,
        RenderCallback,
    ));
}

fn flush_buffer(project: &mut Project, renderstate: &RenderState, rect: Rect, ctx: &egui::Context) {
    let camera_dir = project
        .state
        .camera
        .target
        .sub(project.state.camera.position)
        .normalize();

    project.state.uniform_buffer.write(
        &renderstate.queue,
        0,
        &[
            project.state.camera.viewport.width(),
            project.state.camera.viewport.height(),
            rect.top() * ctx.pixels_per_point(),
            camera_dir.x,
            camera_dir.y,
            camera_dir.z,
            project.state.hover_pos.x,
            project.state.camera.viewport.height() - project.state.hover_pos.y,
            project.state.camera.ray.origin.x,
            project.state.camera.ray.origin.y,
            project.state.camera.ray.origin.z,
            project.state.camera.ray.direction.x,
            project.state.camera.ray.direction.y,
            project.state.camera.ray.direction.z,
        ],
    );
    project.state.uniform_buffer.write_mat(
        &renderstate.queue,
        4 * 16,
        &project.state.camera.projection_view_matrix,
    );
}

fn run_compute_pass(renderstate: &RenderState) {
    let mut encoder = renderstate
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Compute Encoder"),
        });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute Pass"),
        });

        let mut writer = renderstate.renderer.write();
        let appstate: &mut AppState = writer.callback_resources.get_mut().unwrap();
        let project = &appstate.projects[appstate.selected_project];
        appstate.renderer.compute(pass, project);
    }
    renderstate.queue.submit(Some(encoder.finish()));
}

fn tabcontrolheader(renderstate: &RenderState, ui: &mut egui::Ui) {
    let mut writer = renderstate.renderer.write();
    let appstate: &mut AppState = writer.callback_resources.get_mut().unwrap();
    tabcontrol::show_tabs(
        ui,
        &mut appstate.projects,
        &mut appstate.selected_project,
        |p: &Project| p.name.clone(),
        || Project::new(&renderstate.device, &renderstate.queue),
    );
}

fn update_camera(project: &mut ProjectState, rect: Rect, response: Response, ctx: &egui::Context) {
    if let Some(pos) = response.hover_pos() {
        project.hover_pos = glam::vec2(
            pos.x * ctx.pixels_per_point(),
            pos.y * ctx.pixels_per_point() - rect.top() * ctx.pixels_per_point(),
        );
    }

    project.camera.viewport = Rect {
        min: Pos2 {
            x: rect.min.x * ctx.pixels_per_point(),
            y: rect.min.y * ctx.pixels_per_point(),
        },
        max: Pos2 {
            x: rect.max.x * ctx.pixels_per_point(),
            y: rect.max.y * ctx.pixels_per_point(),
        },
    };

    project.camera.update_input(ctx);
    project.camera.calculate_matrixs();

    let pos = ctx.input(|e| e.pointer.hover_pos()).unwrap_or(Pos2::ZERO);
    project.camera.update_ray(vec2(
        pos.x * ctx.pixels_per_point(),
        pos.y * ctx.pixels_per_point(),
    ));
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


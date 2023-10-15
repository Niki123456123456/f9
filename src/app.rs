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
    commands::command::{get_commands, Command},
    project::{self, Project, ProjectState},
    rendering::{
        buffer_reader::BufferReader,
        renderer::{self, Renderer},
    },
    ui::{main_menu::draw_commands, tabcontrol}, components::component::HoverElement,
};

pub struct AppState {
    pub projects: Vec<Project>,
    pub selected_project: usize,
    pub renderer: Renderer,
}
pub struct App {
    pub adapter_info: AdapterInfo,
    pub last_render_time: Instant,
    pub selected_project: usize,
    pub commands: Vec<Command>,
    pub buffer_reader: BufferReader,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let wgpu_render_state = cc.wgpu_render_state.as_ref().unwrap();
        let device = &wgpu_render_state.device;

        let app = App {
            adapter_info: wgpu_render_state.adapter.get_info(),
            last_render_time: Instant::now(),
            selected_project: 0,
            commands: get_commands(),
            buffer_reader: BufferReader::new(device, &wgpu_render_state.queue, 1_000_000),
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
                {
                    let mut writer = renderstate.renderer.write();
                    let appstate: &mut AppState = writer.callback_resources.get_mut().unwrap();
                    let project = &mut appstate.projects[appstate.selected_project];

                    let data = self.buffer_reader.read_buffer(
                        &project.state.uniform_buffer.atomic_buffer,
                        0,
                        32,
                    );
                    let counter: u32 = ((data[3] as u32) << 24)
                        | ((data[2] as u32) << 16)
                        | ((data[1] as u32) << 8)
                        | (data[0] as u32);

                    let hover_elements: Vec<HoverElement> = self.buffer_reader.read_buffer_gen(
                            &project.state.uniform_buffer.hover_buffer,
                            0,
                            counter as u64,
                        );

                    print!("hover: {} ", hover_elements.len());
                    for hover in hover_elements.iter()  {
                        print!("{:?} ", hover.ctype)
                    }
                    println!("");
                }

                run_render_pass(ui, rect);
            });

        egui::Window::new("GPU Info").show(&ctx, |ui| {
            ui.label(format!("backend: {:?}", &self.adapter_info.backend));
            ui.label(format!("name: {}", &self.adapter_info.name));
            ui.label(format!("device: {}", &self.adapter_info.device));
            ui.label(format!("device_type: {:?}", &self.adapter_info.device_type));
            ui.label(format!("driver: {}", &self.adapter_info.driver));
            ui.label(format!("driver_info: {}", &self.adapter_info.driver_info));
            let ele = self.last_render_time.elapsed();
            let fps = 1.0 / ele.as_secs_f64();
            ui.label(format!("duration {:.0}ms", ele.as_millis() as f64));
            ui.label(format!("frames {:.0}/s", fps));
            self.last_render_time = Instant::now();
        });

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

    project
        .state
        .uniform_buffer
        .clear_hover_counter(&renderstate.queue);
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

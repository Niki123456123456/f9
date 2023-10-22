use eframe::{
    egui_wgpu::wgpu::util::DeviceExt,
    egui_wgpu::{self, wgpu, CallbackResources, CallbackTrait, RenderState},
    wgpu::{
        AdapterInfo, BindGroupLayout, Buffer, BufferDescriptor, BufferUsages, Device, Queue,
        RenderPassColorAttachment, RenderPassDescriptor,
    },
};
use egui::{
    epaint::Shadow, vec2, Align, Color32, Id, LayerId, Layout, Margin, Pos2, Rect, Response,
    Rounding, Stroke, Ui, Vec2,
};
use glam::{Mat4, Vec3};
use instant::{Duration, Instant};
use log::{info, warn};
use std::ops::DerefMut;
use std::{
    num::NonZeroU64,
    ops::Sub,
    sync::{Arc, Mutex},
};

use crate::{
    camera::{self, Camera},
    commands::command::{get_commands, Command},
    components::component::HoverElement,
    project::{self, Project, ProjectState},
    rendering::{
        buffer_reader::{execute, BufferReader},
        renderer::{self, Renderer},
    },
    ui::{main_menu::draw_commands, tabcontrol},
};

pub struct AppState {
    pub projects: Vec<Project>,
    pub selected_project: usize,
    pub renderer: Renderer,
    pub commands: Vec<Command>,
    pub buffer_reader: BufferReader,
}
pub struct App {
    pub adapter_info: AdapterInfo,
    pub last_render_time: Instant,
    pub selected_project: usize,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let wgpu_render_state = cc.wgpu_render_state.as_ref().unwrap();
        let device = &wgpu_render_state.device;

        let app = App {
            adapter_info: wgpu_render_state.adapter.get_info(),
            last_render_time: Instant::now(),
            selected_project: 0,
        };

        let renderer = crate::rendering::renderer::Renderer::new(device, wgpu_render_state);
        wgpu_render_state
            .renderer
            .write()
            .callback_resources
            .insert(AppState {
                projects: vec![Project::new(device, &wgpu_render_state.queue, &renderer)],
                selected_project: 0,
                renderer,
                commands: get_commands(),
                buffer_reader: BufferReader::new(device, &wgpu_render_state.queue, 1_000_000),
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

pub async fn update_async(ctx: &egui::Context, renderstate: &RenderState) {
    let available_rect = ctx.available_rect();
    let layer_id = LayerId::background();
    let id = Id::new("central_panel");

    let clip_rect = ctx.screen_rect();
    let mut panel_ui = Ui::new(ctx.clone(), layer_id, id, available_rect, clip_rect);

    let panel_rect = panel_ui.available_rect_before_wrap();
    let mut panel_ui = panel_ui.child_ui(panel_rect, Layout::top_down(Align::Min));

    let mut prepared = egui::Frame {
        inner_margin: Margin::same(0.),
        outer_margin: Margin::same(0.),
        rounding: Rounding::none(),
        shadow: Shadow::NONE,
        fill: Color32::TRANSPARENT,
        stroke: Stroke::NONE,
    }
    .begin(&mut panel_ui);
    let  ui = &mut  prepared.content_ui;
    ui.expand_to_include_rect(ui.max_rect()); // Expand frame to include it all

    let spacing = ui.spacing_mut();
    spacing.item_spacing = Vec2::ZERO;

    tabcontrolheader(renderstate, ui);

    {
        let mut writer = renderstate.renderer.write();
        let appstate: &mut AppState = writer.callback_resources.get_mut().unwrap();
        let project = &mut appstate.projects[appstate.selected_project];

        draw_commands(ui, project, &appstate.commands);
    }

    let (rect, response) = ui.allocate_at_least(ui.available_size(), egui::Sense::drag());

    {
        let mut writer = renderstate.renderer.write();
        let appstate: &mut AppState = writer.callback_resources.get_mut().unwrap();
        let project = &mut appstate.projects[appstate.selected_project];

        update_camera(&mut project.state, rect, response, ctx);
        flush_buffer(project, renderstate, rect, ctx);
    }
    run_compute_pass(renderstate);
    update_selected(renderstate, ctx).await;

    run_render_pass(ui, rect);

    let response = prepared.end(&mut panel_ui);

    /*
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
    });*/

    ctx.request_repaint();
}

async fn update_selected(renderstate: &RenderState, ctx: &egui::Context) {
    let mut writer = renderstate.renderer.write();
    let appstate: &mut AppState = writer.callback_resources.get_mut().unwrap();
    let project = &mut appstate.projects[appstate.selected_project];

    let data = appstate
        .buffer_reader
        .read_buffer(&project.state.uniform_buffer.atomic_buffer, 0, 32)
        .await;
    if data.len() > 0 {
        let counter: u32 = ((data[3] as u32) << 24)
            | ((data[2] as u32) << 16)
            | ((data[1] as u32) << 8)
            | (data[0] as u32);

        project.state.components.hovers = appstate
            .buffer_reader
            .read_buffer_gen(
                &project.state.uniform_buffer.hover_buffer,
                0,
                counter as u64,
            )
            .await;
        warn!("hover");
        print!("hover: {} ", project.state.components.hovers.len());
        for hover in project.state.components.hovers.iter() {
            print!("{:?} ", hover.ctype)
        }
        println!("");

        project.state.components.update_selected(ctx);
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {}

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let ctx = ctx.clone();
        let renderstate = _frame.wgpu_render_state().unwrap().clone();
        execute(async move {
            update_async(&ctx, &renderstate).await;
        });
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
        || Project::new(&renderstate.device, &renderstate.queue, &appstate.renderer),
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

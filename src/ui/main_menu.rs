use egui::{pos2, vec2, Color32, Mesh, Rect, Sense, Shape, Ui};

use crate::{commands::command::Command, project::Project};

use super::icons::get_icons;

pub fn draw_commands(ui: &mut Ui, project: &mut Project, commands: &Vec<Command>) {
    ui.horizontal(|ui| {
        for command in commands.iter() {
            draw_command(ui, project, command);
        }
    });
}

fn draw_command(ui: &mut Ui, project: &mut Project, command: &Command) {
    let (rect, resp) = ui.allocate_at_least(vec2(40., 40.), Sense::click());
    
    if resp.hovered() {
        
    }

    let icons = get_icons(ui);
    let texture_id = (command.get_icon)(&icons).id();

    let mut mesh = Mesh::with_texture(texture_id);
    mesh.add_rect_with_uv(
        rect,
        Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
        Color32::WHITE,
    );
    ui.painter().add(Shape::mesh(mesh));

    if resp.clicked() {
        command.function.start(project.sender.clone(), project);
    }
}

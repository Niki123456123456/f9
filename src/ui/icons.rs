use std::sync::Mutex;

use egui::{Ui, TextureHandle};

#[derive(Clone)]
pub struct IconCollection {
    pub visible: TextureHandle,
    pub invisible: TextureHandle,
    pub image: TextureHandle,
    pub home: TextureHandle,
    pub mov: TextureHandle,

    pub draw_bezier: TextureHandle,
    pub draw_circle: TextureHandle,
    pub draw_line: TextureHandle,
    pub draw_rect: TextureHandle,
    pub draw_rect_center: TextureHandle,

    pub change_projection: TextureHandle,
    pub back_view: TextureHandle,
    pub bottom_view: TextureHandle,
    pub front_view: TextureHandle,
    pub left_view: TextureHandle,
    pub right_view: TextureHandle,
    pub top_view: TextureHandle,
}

static ICONS: Mutex<Option<IconCollection>> = Mutex::new(None);

pub fn get_icons(ui: &mut Ui) -> IconCollection {
    let mut icons = ICONS.lock().unwrap();
    if icons.is_none() {
        *icons = Some(IconCollection {

            visible: load_svg(ui, "visible", include_bytes!("../../assets/icons/visible.svg")),
            invisible: load_svg(ui, "invisible", include_bytes!("../../assets/icons/invisible.svg")),
            image: load_svg(ui, "invisible", include_bytes!("../../assets/icons/image.svg")),
            home: load_svg(ui, "invisible", include_bytes!("../../assets/icons/home.svg")),
            mov: load_svg(ui, "invisible", include_bytes!("../../assets/icons/move.svg")),

            draw_bezier: load_image(ui, "invisible", include_bytes!("../../assets/icons/draw_bezier.png")),
            draw_circle: load_image(ui, "invisible", include_bytes!("../../assets/icons/draw_circle.png")),
            draw_line: load_image(ui, "invisible", include_bytes!("../../assets/icons/draw_line.png")),
            draw_rect: load_image(ui, "invisible", include_bytes!("../../assets/icons/draw_rect.png")),
            draw_rect_center: load_image(ui, "invisible", include_bytes!("../../assets/icons/draw_rect_center.png")),

            change_projection: load_svg(ui, "invisible", include_bytes!("../../assets/icons/change_projection.svg")),
            back_view: load_svg(ui, "invisible", include_bytes!("../../assets/icons/views/back_view.svg")),
            bottom_view: load_svg(ui, "invisible", include_bytes!("../../assets/icons/views/bottom_view.svg")),
            front_view: load_svg(ui, "invisible", include_bytes!("../../assets/icons/views/front_view.svg")),
            left_view: load_svg(ui, "invisible", include_bytes!("../../assets/icons/views/left_view.svg")),
            right_view: load_svg(ui, "invisible", include_bytes!("../../assets/icons/views/right_view.svg")),
            top_view: load_svg(ui, "invisible", include_bytes!("../../assets/icons/views/top_view.svg")),
        });
    }
   
    return icons.as_ref().unwrap().clone();
}

fn load_image(ui: &mut Ui, name : &'static str, image_bytes : &[u8]) -> TextureHandle {
    let tex= ui.ctx().load_texture(
        name,
        egui_extras::image::load_image_bytes(image_bytes).unwrap(),
        Default::default()
    );
    return tex;
}

fn load_svg(ui: &mut Ui, name : &'static str, image_bytes : &[u8]) -> TextureHandle {
    let image = egui_extras::image::load_svg_bytes_with_size(image_bytes, egui_extras::image::FitTo::Size(40, 40)).unwrap();
    let tex= ui.ctx().load_texture(
        name,
        image,
        Default::default()
    );
    return tex;
}
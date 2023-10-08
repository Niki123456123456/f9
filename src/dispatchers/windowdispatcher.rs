use crate::project::ProjectState;
use egui::{Ui, Context};

use super::dispatcher::Dispatcher;

pub struct WindowDispatcher {
    pub window_name: String,
    pub func: Box<dyn Fn(&mut ProjectState, &mut Ui) -> () + Send + Sync>,
}

impl Dispatcher for WindowDispatcher {
    fn draw2d(&mut self, state: &mut ProjectState, ctx: &Context) {
        egui::Window::new(self.window_name.clone())
        .scroll2([true, true])
        .show(ctx, |ui| {
            (self.func)(state, ui);
        });
    }
}
impl WindowDispatcher {
    pub fn new( window_name : &'static str, func: impl Fn(&mut ProjectState, &mut Ui) -> () + 'static + Send + Sync) -> Self {
        Self {
            window_name : window_name.into(),
            func: Box::new(func),
        }
    }
}

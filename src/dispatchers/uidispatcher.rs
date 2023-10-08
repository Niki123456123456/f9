use crate::core::result::Result;
use crate::project::ProjectState;
use async_std::channel::Receiver;
use async_std::channel::Sender;
use egui::Ui;

use super::dispatcher::Dispatcher;

pub struct UiDispatcher<T> {
    pub sender: Sender<Result<T>>,
    pub receiver: Receiver<Result<T>>,
    pub func: Box<dyn Fn(&mut ProjectState, &mut Ui) -> Option<Result<T>> + Send + Sync>,
}

impl<T> Dispatcher for UiDispatcher<T> {
    fn draw2d_nointeract(&mut self, state: &mut ProjectState, ui: &mut Ui) {
        if let Some(result) = (self.func)(state, ui) {
            let _ = self.sender.try_send(result);
        }
    }
}
impl<T> UiDispatcher<T> {
    pub fn new(
        func: impl Fn(&mut ProjectState, &mut Ui) -> Option<Result<T>> + 'static + Send + Sync,
    ) -> Self {
        let (s, r): (Sender<Result<T>>, Receiver<Result<T>>) = async_channel::bounded(1);
        Self {
            sender: s,
            receiver: r,
            func: Box::new(func),
        }
    }
}

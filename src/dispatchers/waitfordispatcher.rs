use crate::core::result::Result;
use crate::project::ProjectState;
use async_std::channel::Receiver;
use async_std::channel::Sender;

use super::dispatcher::Dispatcher;

pub struct WaitForDispatcher<T> {
    pub sender: Sender<Result<T>>,
    pub receiver: Receiver<Result<T>>,
    pub func: Box<dyn Fn(&mut ProjectState) -> Option<Result<T>> + Send + Sync>,
}

impl<T> Dispatcher for WaitForDispatcher<T> {
    fn interact(&mut self, state: &mut ProjectState) {
        if let Some(result) = (self.func)(state) {
            let _ = self.sender.try_send(result);
        }
    }
}
impl<T> WaitForDispatcher<T> {
    pub fn new(func: impl Fn(&mut ProjectState) -> Option<Result<T>> + 'static + Send + Sync) -> Self {
        let (s, r): (Sender<Result<T>>, Receiver<Result<T>>) = async_channel::bounded(1);
        Self {
            sender: s,
            receiver: r,
            func: Box::new(func),
        }
    }
}

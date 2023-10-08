use crate::project::ProjectState;
use async_std::channel::Receiver;
use async_std::channel::Sender;

use super::dispatcher::Dispatcher;

pub struct OnceDispatcher<T> {
    pub has_run : bool,
    pub sender: Sender<T>,
    pub receiver: Receiver<T>,
    pub func: Box<dyn Fn(&mut ProjectState) -> T + Send + Sync>,
}

impl<T> Dispatcher for OnceDispatcher<T> {
    fn interact(&mut self, state: &mut ProjectState) {
        if self.has_run == false {
            let result = (self.func)(state);
            let _ = self.sender.try_send(result);
            self.has_run = true;
        }
    }
}
impl<T> OnceDispatcher<T> {
    pub fn new(func: impl Fn(&mut ProjectState) -> T + 'static + Send + Sync) -> Self {
        let (s, r): (Sender<T>, Receiver<T>) = async_channel::bounded(1);
        Self {
            has_run : false,
            sender: s,
            receiver: r,
            func: Box::new(func),
        }
    }
}

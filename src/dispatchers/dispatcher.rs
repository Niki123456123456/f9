use std::sync::Mutex;

use crate::project::ProjectState;
use crate::{components::point, core::result::Result};
use async_std::channel::Sender;
use egui::{Context, Ui};
use glam::Vec3;
use uuid::Uuid;

use super::{
    oncedispatcher::OnceDispatcher, uidispatcher::UiDispatcher,
    waitfordispatcher::WaitForDispatcher, windowdispatcher::WindowDispatcher,
};

pub enum DispatcherEvent {
    Add(Uuid, Disp),
    Remove(Uuid),
}

pub type Disp = Box<dyn Dispatcher + Send + Sync>;

pub trait Dispatcher {
    fn interact(&mut self, state: &mut ProjectState) {}
    fn draw2d(&mut self, state: &mut ProjectState, ctx: &Context) {}
    fn draw2d_nointeract(&mut self, state: &mut ProjectState, ui: &mut Ui) {}
}

pub async fn wait_for_ui<T>(
    sender: Sender<DispatcherEvent>,
    func: impl Fn(&mut ProjectState, &mut Ui) -> Option<Result<T>> + 'static + Send + Sync,
) -> Result<T>
where
    T: Send + 'static,
{
    let dispatcher_id = Uuid::new_v4();
    let dispatcher = UiDispatcher::new(func);
    let recv = dispatcher.receiver.clone();
    let _ = sender.try_send(DispatcherEvent::Add(dispatcher_id, Box::new(dispatcher)));
    let result = recv.recv().await.unwrap();
    let _ = sender.try_send(DispatcherEvent::Remove(dispatcher_id));
    return result;
}

pub async fn wait_for<T>(
    sender: Sender<DispatcherEvent>,
    func: impl Fn(&mut ProjectState) -> Option<Result<T>> + 'static + Send + Sync,
) -> Result<T>
where
    T: Send + 'static,
{
    let dispatcher_id = Uuid::new_v4();
    let dispatcher = WaitForDispatcher::new(func);
    let recv = dispatcher.receiver.clone();
    let _ = sender.try_send(DispatcherEvent::Add(dispatcher_id, Box::new(dispatcher)));
    let result = recv.recv().await.unwrap();
    let _ = sender.try_send(DispatcherEvent::Remove(dispatcher_id));
    return result;
}

pub async fn once<T>(
    sender: Sender<DispatcherEvent>,
    func: impl Fn(&mut ProjectState) -> T + 'static + Send + Sync,
) -> T
where
    T: Send + 'static,
{
    let dispatcher_id = Uuid::new_v4();
    let dispatcher = OnceDispatcher::new(func);
    let recv = dispatcher.receiver.clone();
    let _ = sender.try_send(DispatcherEvent::Add(dispatcher_id, Box::new(dispatcher)));
    let result = recv.recv().await.unwrap();
    let _ = sender.try_send(DispatcherEvent::Remove(dispatcher_id));
    return result;
}

pub fn window(
    sender: Sender<DispatcherEvent>,
    window_name: &'static str,
    func: impl Fn(&mut ProjectState, &mut Ui) -> () + 'static + Send + Sync,
) -> Uuid {
    let dispatcher_id = Uuid::new_v4();
    let dispatcher = WindowDispatcher::new(window_name, func);
    let _ = sender.try_send(DispatcherEvent::Add(dispatcher_id, Box::new(dispatcher)));
    return dispatcher_id;
}

pub fn remove(sender: Sender<DispatcherEvent>, dispatcher_id: Uuid) {
    let _ = sender.try_send(DispatcherEvent::Remove(dispatcher_id));
}

pub async fn select_point(
    sender: Sender<DispatcherEvent>, create_point : bool,
    func: impl Fn(&mut ProjectState, usize, Vec3) -> () + 'static + Send + Sync,
) -> Result<(usize, Vec3)> {
    let p = Mutex::new(None);
    wait_for(sender.clone(), move |state| {
        let pos = state.camera.world_mouse_position;

        let mut p = p.lock().unwrap();
        
        let point = if create_point { state.components.points.push_or_update(
            &mut p,
            || point::new(pos),
            |x| {
                x.data.position = pos;
            },
        )} else {0};

        func(state, point, pos);

        if state.is_mouse_clicked {
            return Some(Ok((point, pos)));
        }
        return None;
    })
    .await
}

use async_std::channel::Sender;
use egui::{TextureHandle, Key};
use crate::{project::Project,  dispatchers::dispatcher::{DispatcherEvent}, ui::icons::IconCollection,};

use super::{view_commands::{RightView, LeftView, BackView, ChangeProjection, HomeView, TopView, BottomView, FrontView}};


pub fn get_commands() -> Vec<Command> {
    let commands: Vec<Command> = vec![
        Command {
            id: uuid::Uuid::new_v4(),
            name: "Change Projection".into(),
            down_keys: vec![],
            released_key: Some(Key::Num0),
            function: Box::new(ChangeProjection),
            get_icon: Box::new(|x| &x.change_projection),
        },
        Command {
            id: uuid::Uuid::new_v4(),
            name: "Home View".into(),
            down_keys: vec![],
            released_key: Some(Key::Num1),
            function: Box::new(HomeView),
            get_icon: Box::new(|x| &x.home),
        },
        Command {
            id: uuid::Uuid::new_v4(),
            name: "Top View".into(),
            down_keys: vec![],
            released_key: Some(Key::Num8),
            function: Box::new(TopView),
            get_icon: Box::new(|x| &x.top_view),
        },
        Command {
            id: uuid::Uuid::new_v4(),
            name: "Bottom View".into(),
            down_keys: vec![],
            released_key: Some(Key::Num2),
            function: Box::new(BottomView),
            get_icon: Box::new(|x| &x.bottom_view),
        },
        Command {
            id: uuid::Uuid::new_v4(),
            name: "Front View".into(),
            down_keys: vec![],
            released_key: Some(Key::Num4),
            function: Box::new(FrontView),
            get_icon: Box::new(|x| &x.front_view),
        },
        Command {
            id: uuid::Uuid::new_v4(),
            name: "Back View".into(),
            down_keys: vec![],
            released_key: Some(Key::Num6),
            function: Box::new(BackView),
            get_icon: Box::new(|x| &x.back_view),
        },
        Command {
            id: uuid::Uuid::new_v4(),
            name: "Left View".into(),
            down_keys: vec![],
            released_key: Some(Key::Num5),
            function: Box::new(LeftView),
            get_icon: Box::new(|x| &x.left_view),
        },
        Command {
            id: uuid::Uuid::new_v4(),
            name: "Right View".into(),
            down_keys: vec![],
            released_key: None,
            function: Box::new(RightView),
            get_icon: Box::new(|x| &x.right_view),
        },
    ];

    return commands;
}

pub trait CommandFunction {
    fn start(&self, sender: Sender<DispatcherEvent>, project : &mut Project);
}

pub struct Command {
    pub id: uuid::Uuid,
    pub name: String,
    pub down_keys: Vec<egui::Key>,
    pub released_key: Option<egui::Key>,
    pub function: Box<dyn CommandFunction + Send + Sync>,
    pub get_icon: Box<dyn Fn(&IconCollection) -> (&TextureHandle) + Send + Sync>,
}

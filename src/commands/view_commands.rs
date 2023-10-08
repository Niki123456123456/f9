use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

use crate::{ dispatchers::dispatcher::DispatcherEvent, project::Project};
use async_std::channel::Sender;
use glam::Vec3;

use super::command::CommandFunction;



pub struct ChangeProjection;
pub struct HomeView;
pub struct TopView;
pub struct BottomView;
pub struct FrontView;
pub struct BackView;
pub struct LeftView;
pub struct RightView;

impl CommandFunction for ChangeProjection {
    fn start(&self, _: Sender<DispatcherEvent>, project: &mut Project) {
        match project.state.camera.projection {
            crate::camera::Projection::Perspective => {
                project.state.camera.projection = crate::camera::Projection::Orthographics;
            },
            crate::camera::Projection::Orthographics => {
                project.state.camera.projection = crate::camera::Projection::Perspective;
            },
        }
    }
}

impl CommandFunction for HomeView {
    fn start(&self, _: Sender<DispatcherEvent>, project: &mut Project) {
        project.state.camera.x_angle = -FRAC_PI_4;
        project.state.camera.y_angle = -FRAC_PI_4;
        project.state.camera.plane.orientation = Vec3::Y;
    }
}
impl CommandFunction for TopView {
    fn start(&self, _: Sender<DispatcherEvent>, project: &mut Project) {
        project.state.camera.x_angle = 0.00000001;
        project.state.camera.y_angle = 0.0;
        project.state.camera.plane.orientation = Vec3::Y;
    }
}
impl CommandFunction for BottomView {
    fn start(&self, _: Sender<DispatcherEvent>, project: &mut Project) {
        project.state.camera.x_angle = -PI + 0.00000001;
        project.state.camera.y_angle = 0.0;
        project.state.camera.plane.orientation = Vec3::Y;
    }
}
impl CommandFunction for FrontView {
    fn start(&self, _: Sender<DispatcherEvent>, project: &mut Project) {
        project.state.camera.x_angle = -FRAC_PI_2;
        project.state.camera.y_angle = 0.0;
        project.state.camera.plane.orientation = Vec3::X;
    }
}
impl CommandFunction for BackView {
    fn start(&self, _: Sender<DispatcherEvent>, project: &mut Project) {
        project.state.camera.x_angle = -FRAC_PI_2;
        project.state.camera.y_angle = -PI;
        project.state.camera.plane.orientation = Vec3::X;
    }
}
impl CommandFunction for LeftView {
    fn start(&self, _: Sender<DispatcherEvent>, project: &mut Project) {
        project.state.camera.x_angle = -FRAC_PI_2;
        project.state.camera.y_angle = -FRAC_PI_2;
        project.state.camera.plane.orientation = Vec3::Z;
    }
}
impl CommandFunction for RightView {
    fn start(&self, _: Sender<DispatcherEvent>, project: &mut Project) {
        project.state.camera.x_angle = -FRAC_PI_2;
        project.state.camera.y_angle = FRAC_PI_2;
        project.state.camera.plane.orientation = Vec3::Z;
    }
}

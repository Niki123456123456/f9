// 1 = true, 0 = false
use crate::component_collection::ComponentCollection;

use glam::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum IndexPosition {
    First = 0,
    Last = 1,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum WalkDirection {
    Down = 0,
    Up = 1,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub enum ComponentFlags {
    None = 0,
    Visible = 1,
    Hover = 2,
    Selected = 4,
    Deleted = 8,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ComponentIdentifier {
    pub index: u32,
    pub ctype: ComponentType,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct HoverElement {
    pub index: u32,
    pub ctype: ComponentType,
    pub distance: f32,
    pub position: Vec3,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub enum ComponentType {
    Point = 1,
    Line = 2,
    Circle = 3,
    Bezier = 4,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Component<T> {
    pub data: T,
    pub flags: i32,
}

impl<T> Component<T> {
    pub fn notvisible(mut self) -> Self {
        self.flags = self.flags & (!(ComponentFlags::Visible as i32));
        return self;
    }

    pub fn new(data: T) -> Self {
        Component {
            data,
            flags: ComponentFlags::Visible as i32,
        }
    }
}

impl<T> IComponent for Component<T>
where
    T: IComponentData + Clone + 'static,
{
    fn hover(&mut self) {
        self.flags = self.flags | ComponentFlags::Hover as i32;
    }
    fn nothover(&mut self) {
        self.flags = self.flags & (!(ComponentFlags::Hover as i32));
    }
    fn visible(&mut self) {
        self.flags = self.flags | ComponentFlags::Visible as i32;
    }
    fn invisible(&mut self) {
        self.flags = self.flags & (!(ComponentFlags::Visible as i32));
    }
    fn selected(&mut self) {
        self.flags = self.flags | ComponentFlags::Selected as i32;
    }
    fn deselected(&mut self) {
        self.flags = self.flags & (!(ComponentFlags::Selected as i32));
    }
    fn is_selected(&self) -> bool {
        (self.flags & (ComponentFlags::Selected as i32)) == (ComponentFlags::Selected as i32)
    }

    fn get_center(&self, components: &ComponentCollection) -> Vec3 {
        self.data.get_center(components)
    }

    fn move_dir(&mut self, dir: Vec3) {
        self.data.move_dir(dir);
    }
    fn get_orientation(&self, components: &ComponentCollection, direction: WalkDirection) -> Vec3 {
        self.data.get_orientation(components, direction)
    }
    fn get_position(
        &self,
        components: &ComponentCollection,
        direction: WalkDirection,
        t: f32,
    ) -> Vec3 {
        self.data.get_position(components, direction, t)
    }
    fn get_index(&self, direction: WalkDirection, position: IndexPosition, self_index: u32) -> Option<u32> {
        self.data.get_index(direction, position, self_index)
    }

    fn clon(&self) -> Box<dyn IComponent> {
        return Box::new(Component {
            data: self.data.clone(),
            flags: self.flags,
        });
    }
}

pub trait IComponentData {
    fn get_center(&self, components: &ComponentCollection) -> Vec3;
    fn move_dir(&mut self, dir: Vec3) {}
    fn get_orientation(&self, components: &ComponentCollection, direction: WalkDirection) -> Vec3 {
        Vec3::ZERO
    }
    fn get_position(
        &self,
        components: &ComponentCollection,
        direction: WalkDirection,
        t: f32,
    ) -> Vec3 {
        Vec3::ZERO
    }
    fn get_index(&self, direction: WalkDirection, position: IndexPosition, self_index: u32) -> Option<u32> {
        None
    }
}

pub trait IComponent {
    fn hover(&mut self);
    fn nothover(&mut self);
    fn visible(&mut self);
    fn invisible(&mut self);
    fn selected(&mut self);
    fn deselected(&mut self);
    fn is_selected(&self) -> bool;
    fn get_center(&self, components: &ComponentCollection) -> Vec3;
    fn move_dir(&mut self, dir: Vec3);
    fn get_orientation(&self, components: &ComponentCollection, direction: WalkDirection) -> Vec3;
    fn get_position(
        &self,
        components: &ComponentCollection,
        direction: WalkDirection,
        t: f32,
    ) -> Vec3;
    fn end_position(&self, components: &ComponentCollection, direction: WalkDirection, position: IndexPosition) -> Vec3 {
        match position {
            IndexPosition::First => self.get_position(components, direction, 0.),
            IndexPosition::Last => self.get_position(components, direction, 1.),
        }
    }
    fn get_index(&self, direction: WalkDirection, position: IndexPosition, self_index: u32) -> Option<u32>;

    fn clon(&self) -> Box<dyn IComponent>;
}

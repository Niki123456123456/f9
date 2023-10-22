use std::{array, sync::Arc};

use eframe::wgpu::{self, BufferUsages, Device, Queue};

use crate::components::{
    bezier::Bezier, circle::Circle, component::{Component, HoverElement, ComponentType, IComponent, IComponentData, ComponentIdentifier}, line::Line, point::Point, vertex::Vertex,
};

pub struct ComponentCollection {
    pub hovers: Vec<HoverElement>,
    pub selected: Vec<ComponentIdentifier>,

    pub axises: ComponentArray<Vertex>,
    pub grids: ComponentArray<Vertex>,
    pub arrows: ComponentArray<Vertex>,
    pub arrow_planes: ComponentArray<Vertex>,

    pub points: ComponentArray<Point>,
    pub lines: ComponentArray<Line>,
    pub beziers: ComponentArray<Bezier>,
    pub circles: ComponentArray<Circle>,
}

impl ComponentCollection {
    pub fn get_most_hovered(&mut self) -> Option<HoverElement> {
        if let Some(x) = self.hovers.first() {
            let threshold = 0.01;
            let mut ctype = x.ctype;
            let mut index = 0;
            let mut distance = x.distance;

            for (i, element) in self.hovers.iter().enumerate() {
                let current_distance = element.distance;
                let diff: f32 = (current_distance - distance).abs();
                if ctype == ComponentType::Point {
                    if element.ctype == ComponentType::Point {
                        if current_distance < distance {
                            ctype = element.ctype;
                            index = i;
                            distance = element.distance;
                        }
                    } else if current_distance < distance && diff > threshold {
                        ctype = element.ctype;
                        index = i;
                        distance = element.distance;
                    }
                } else {
                    if element.ctype == ComponentType::Point {
                        if current_distance < distance || diff <= threshold {
                            ctype = element.ctype;
                            index = i;
                            distance = element.distance;
                        }
                    } else if current_distance < distance {
                        ctype = element.ctype;
                        index = i;
                        distance = element.distance;
                    }
                }
            }

            for (i, element) in self.hovers.clone().iter().enumerate() {
                if i != index {
                    self.update_c(element.ctype, element.index as usize, |c|{c.nothover()});
                }
            }
            return Some(self.hovers[index as usize].clone());
        }
        return None;
    }

    pub fn update_selected(&mut self, ctx: &egui::Context) {
        if let Some(hover) = self.get_most_hovered() {
            ctx.input(|i| {
                let identifier = ComponentIdentifier {
                    index: hover.index,
                    ctype: hover.ctype,
                };
                if i.pointer.primary_clicked() {
                    if self.is_selected(hover.ctype, hover.index as usize) {
                        if i.modifiers.ctrl {
                            if let Some(index) = self
                                .selected
                                .iter()
                                .position(|i| i.index == hover.index && i.ctype == hover.ctype)
                            {
                                self.selected.remove(index);
                            }
                            self.update_c(hover.ctype, hover.index as usize, |c| {
                                c.deselected();
                            });
                        } else {
                            for identifer in self.selected.clone().iter() {
                                self.update_c(identifer.ctype, identifer.index as usize, |c| {
                                    c.deselected();
                                });
                            }
                            self.selected = vec![];
                        }
                    } else {
                        if i.modifiers.ctrl {
                            self.selected.push(identifier);
                        } else {
                            for identifer in self.selected.clone().iter() {
                                self.update_c(identifer.ctype, identifer.index as usize, |c| {
                                    c.deselected();
                                });
                            }
                            self.selected = vec![identifier];
                        }
                        self.update_c(hover.ctype, hover.index as usize, |c| {
                            c.selected();
                        });
                    }
                }
            });
        }
    }

    pub fn is_selected(&self, ctype: ComponentType, index: usize) -> bool {
        let mut result = false;
        self.get_c(ctype, index, |c| {
            if let Some(c) = c {
                result = c.is_selected();
            }
        });
        return result;
    }

    pub fn get_c<F,T>(&self, ctype: ComponentType, index: usize, func: F) -> T
    where
        F: FnOnce(Option<Box<&dyn IComponent>>) -> T,
    {
        self.get_array(ctype, |array| {
            let x = array.get_c(index).clone();
            (func)(x)
        })
    }

    pub fn get_array<F,T>(&self, ctype: ComponentType, func: F) -> T
    where
        F: FnOnce(&dyn IComponentArray) -> T,
    {
        match ctype {
            ComponentType::Point => {
                (func)(&self.points)
            }
            ComponentType::Line => {
                (func)(&self.lines)
            }
            ComponentType::Circle => {
                (func)(&self.circles)
            }
            ComponentType::Bezier => {
                (func)(&self.beziers)
            }
            ComponentType::Arrow => {
                (func)(&self.arrows)
            }
            ComponentType::ArrowPlane => {
                (func)(&self.arrow_planes)
            }
        }
    }
    
    pub fn update_c<F>(&mut self, ctype: ComponentType, index: usize, func: F)
    where
        F: FnOnce(&mut dyn IComponent) + 'static,
    {
        self.update_array(ctype, |array| {
            array.update_c(
                index,
                Box::new(|c| {
                    (func)(c);
                }),
            );
        });
    }

    pub fn update_array<F>(&mut self, ctype: ComponentType, func: F)
    where
        F: FnOnce(&mut dyn IComponentArray),
    {
        match ctype {
            ComponentType::Point => {
                (func)(&mut self.points);
            }
            ComponentType::Line => {
                (func)(&mut self.lines);
            }
            ComponentType::Circle => {
                (func)(&mut self.circles);
            }
            ComponentType::Bezier => {
                (func)(&mut self.beziers);
            }
            ComponentType::Arrow => {
                (func)(&mut self.arrows);
            }
            ComponentType::ArrowPlane => {
                (func)(&mut self.arrow_planes);
            }
        };
    }
}

pub trait IComponentArray {
    fn update_c(&mut self, index: usize, func: Box<dyn FnOnce(&mut dyn IComponent)>);

    fn get_c(&self, index: usize) -> Option<Box<&dyn IComponent>>;
}

impl<T> IComponentArray for ComponentArray<T>
where
    T: IComponentData + Clone + 'static,
{
    fn update_c(&mut self, index: usize, func: Box<dyn FnOnce(&mut dyn IComponent)>) {
        self.update(index, |c| {
            (func)(c);
        });
    }

    fn get_c(&self, index: usize) -> Option<Box<&dyn IComponent>> {
        if let Some(c) = self.array.get(index) {
            return Some(Box::new(c));
        }
        return None;
    }
}

pub struct ComponentArray<T> {
    pub array: Vec<Component<T>>,
    pub buffer_size: usize,
    pub buffer: wgpu::Buffer,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
}

impl<T> ComponentArray<T> {
    pub fn new(
        array: Vec<Component<T>>,
        device: &Arc<Device>,
        queue: &Arc<Queue>,
    ) -> ComponentArray<T> {
        let mem_size = core::mem::size_of::<Component<T>>();
        let buffer_size = array.len() * mem_size;

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
            mapped_at_creation: true,
            size: buffer_size as u64,
        });

        unsafe {
            let data: &[u8] = core::slice::from_raw_parts(array.as_ptr() as *const u8, buffer_size);
            buffer
                .slice(0..buffer_size as u64)
                .get_mapped_range_mut()
                .copy_from_slice(data);
        }
        buffer.unmap();

        return ComponentArray {
            array,
            buffer_size,
            buffer,
            device: device.clone(),
            queue: queue.clone(),
        };
    }

    pub fn push_or_update<Y, X>(&mut self, index: &mut Option<usize>, insert: X, update: Y) -> usize
    where
        X: FnOnce() -> Component<T>,
        Y: FnOnce(&mut Component<T>),
    {
        if let Some(index) = index {
            if let Some(()) = self.update(*index, update) {
                return *index;
            }
        }
        let i = self.push((insert)());
        *index = Some(i);
        return i;
    }

    pub fn update<Y, X>(&mut self, index: usize, func: Y) -> Option<X>
    where
        Y: FnOnce(&mut Component<T>) -> X,
    {
        if let Some(component) = self.array.get_mut(index) {
            let x = (func)(component);
            unsafe {
                let single_size = std::mem::size_of::<Component<T>>();
                let offset = (single_size * index) as u64;
                //println!("update {} of {}", size, self.buffer_size);
                let data = std::slice::from_raw_parts(
                    component as *const Component<T> as *const u8,
                    single_size,
                );

                self.queue.write_buffer(&self.buffer, offset, data);
            }
            return Some(x);
        }
        return None;
    }

    fn get_needed_buffer_size(&self) -> usize {
        return self.array.len() * core::mem::size_of::<Component<T>>();
    }

    pub fn push(&mut self, component: Component<T>) -> usize {
        self.array.push(component);

        let needed_buffer_size = self.get_needed_buffer_size();
        if needed_buffer_size > self.buffer_size {
            self.resize_buffer(needed_buffer_size);
        }

        let index = self.array.len() - 1;
        self.update(index, |c| {});
        return index;
    }

    fn resize_buffer(&mut self, new_size: usize) {
        self.buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
            mapped_at_creation: true,
            size: new_size as u64,
        });

        let size = self.array.len() * core::mem::size_of::<Component<T>>();
        unsafe {
            let data: &[u8] = core::slice::from_raw_parts(self.array.as_ptr() as *const u8, size);
            self.buffer
                .slice(0..size as u64)
                .get_mapped_range_mut()
                .copy_from_slice(data);
        }
        self.buffer.unmap();

        //println!("update buffer {} -> {}", self.buffer_size, new_size);
        self.buffer_size = new_size;
    }
}

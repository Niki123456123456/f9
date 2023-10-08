use std::{array, sync::Arc};

use eframe::wgpu::{self, BufferUsages, Device};

use crate::components::{component::Component, vertex::Vertex, point::Point, line::Line, bezier::Bezier, circle::Circle};

pub struct ComponentCollection {
    pub axises: ComponentArray<Vertex>,
    pub grids: ComponentArray<Vertex>,
    pub arrows: ComponentArray<Vertex>,
    pub arrow_planes: ComponentArray<Vertex>,
    pub points: ComponentArray<Point>,
    pub lines: ComponentArray<Line>,
    pub beziers: ComponentArray<Bezier>,
    pub circles: ComponentArray<Circle>,
}

pub struct ComponentArray<T> {
    pub array: Vec<Component<T>>,
    pub buffer_size: usize,
    pub buffer: wgpu::Buffer,
}

impl<T> ComponentArray<T> {
    pub fn new(
        array: Vec<Component<T>>,
        device: &Arc<Device>,
        queue: &wgpu::Queue,
    ) -> ComponentArray<T> {
        let mem_size =core::mem::size_of::<Component<T>>();
        let buffer_size = array.len() * mem_size;

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
            mapped_at_creation: false,
            size: buffer_size as u64,
        });

        unsafe {
            let data: &[u8] = core::slice::from_raw_parts(
                array.as_ptr() as *const u8,
                array.len() * mem_size,
            );
            queue.write_buffer(&buffer, 0, &data);
        }
       

        return ComponentArray {
            array,
            buffer_size,
            buffer,
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
                let size = (single_size * index) as i32;
                //println!("update {} of {}", size, self.buffer_size);
                let data = std::slice::from_raw_parts(
                    component as *const Component<T> as *const u8,
                    single_size,
                );

                // todo write data to buffer
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
        unsafe {
            // todo recreate buffer
        }

        //println!("update buffer {} -> {}", self.buffer_size, new_size);
        self.buffer_size = new_size;
    }
}

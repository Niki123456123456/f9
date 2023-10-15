use std::sync::Arc;

use eframe::wgpu::*;

use crate::core::basics::vec_from_bytes;

pub struct BufferReader {
    pub read_buffer: Buffer,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
}

impl BufferReader {
    pub fn new(device: &Arc<Device>, queue: &Arc<Queue>, size: u64) -> Self {
        let read_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("read_buffer"),
            size,
            usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        Self {
            device: device.clone(),
            queue: queue.clone(),
            read_buffer,
        }
    }

    pub fn read_buffer(&self, source : &Buffer, offset : u64, size: u64) -> Vec<u8> {
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Buffer Copy Encoder"),
            });
        encoder.copy_buffer_to_buffer(source, offset, &self.read_buffer, 0, size);
        self.queue.submit(Some(encoder.finish()));

        let slice = self.read_buffer.slice(0..size);
        let (sender, mut receiver) = futures_channel::oneshot::channel();
        slice.map_async(MapMode::Read, |result| {
            let _ = sender.send(result);
        });
        self.device.poll(Maintain::Wait);
        let mut vec = vec![];
        for i in 0..1000 {
            if let Ok(Some(t)) = receiver.try_recv() {
                let data: &[u8] = &slice.get_mapped_range();
                vec = data.to_vec();
                break;
            }
        }
        self.read_buffer.unmap();

        return vec;
    }

    pub fn read_buffer_gen<T>(&self, source : &Buffer, offset : u64, size: u64) -> Vec<T> {
        if size == 0 {
            return vec![];
        }
        let bytes = self.read_buffer(source, offset, size * std::mem::size_of::<T>() as u64);
        return vec_from_bytes(bytes);
    }

}

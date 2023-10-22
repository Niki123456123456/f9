use std::sync::Arc;

use eframe::wgpu::*;
use instant::Duration;
use instant::Instant;

use crate::core::basics::vec_from_bytes;
use log::log;
use log::Level;
use std::future::Future;
//use pollster::FutureExt as _;

#[cfg(not(target_arch = "wasm32"))]
pub fn execute<F: Future<Output = ()> + 'static>(f: F) {
    // this is stupid... use any executor of your choice instead
    //std::thread::spawn(move || futures::executor::block_on(f));
}
#[cfg(target_arch = "wasm32")]
pub fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}

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

    pub async fn read_buffer(&self, source: &Buffer, offset: u64, size: u64) -> Vec<u8> {
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Buffer Copy Encoder"),
            });
        encoder.copy_buffer_to_buffer(source, offset, &self.read_buffer, 0, size);
        self.queue.submit(Some(encoder.finish()));

        let slice = self.read_buffer.slice(0..size);
        //let (sender, receiver) = async_channel::bounded(1);
        //let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        //let (mut sender, mut receiver) = futures::channel::mpsc::unbounded();
        let (sender, mut receiver) = futures_channel::oneshot::channel::<i32>();
        //let (sender, receiver)= std::sync::mpsc::channel();
        //let (sender, receiver) = std::sync::mpsc::channel();
        slice.map_async(MapMode::Read, move|result| {
            execute(async move {let _ = sender.send(0);});
            
            //sender.start_send(result);
            log!(Level::Error, "Send Data");
        });
        self.device.poll(Maintain::Wait);
        let mut vec = vec![];

        let r = receiver.await;
        /* 
        for x in 0..10000 {
            match receiver.try_recv() {
                Ok(_) => {
                    log!(Level::Error, "receive Data");
                },
                Err(error) => {
                },
            }
        }*/
        //receiver.recv_deadline(std::time::Instant::now());
        //receiver.recv_timeout(Duration::from_millis(20));
        //receiver.recv().unwrap();
/* 
        if let Some(_) = receiver.receive().await {

        }*/

        //let recv = receiver.recv().await;
    
        self.read_buffer.unmap();

        return vec;
    }

    pub async fn read_buffer_gen<T>(&self, source: &Buffer, offset: u64, size: u64) -> Vec<T> {
        if size == 0 {
            return vec![];
        }
        let bytes = self.read_buffer(source, offset, size * std::mem::size_of::<T>() as u64).await;
        return vec_from_bytes(bytes);
    }
}

use crate::ChannelId;
use bytes::Bytes;
use std::collections::VecDeque;
use std::task::{Poll, Waker};

// Max 100 kB
const MAX_QUEUE_SIZE: usize = 100000;

pub struct PacketQueue {
    queue: VecDeque<(ChannelId, Bytes)>,
    size_acc: usize,
    callback: VecDeque<Waker>,
}

impl PacketQueue {
    pub fn enqueue(&mut self, id: ChannelId, data: &[u8], waker: &Waker) -> Poll<usize> {
        let left = MAX_QUEUE_SIZE - self.size_acc;
        let size = data.len().min(2048);
        if left < size {
            self.callback.push_back(waker.clone());
            return Poll::Pending;
        }

        let bytes = Bytes::copy_from_slice(&data[..size]);

        self.queue.push_back((id, bytes));
        self.size_acc += size;

        Poll::Ready(size)
    }
}

mod channel;
mod leb128;
mod packet;
mod queue;

use std::collections::VecDeque;
use std::future::Future;
use std::io::ErrorKind;
use std::pin::Pin;
use crate::queue::PacketQueue;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
use bytes::{Bytes, BytesMut};
use parking_lot::Mutex;
use tokio::net::UdpSocket;
use tokio::sync::{Notify, Semaphore};
use tokio::time::{Instant, Interval, Sleep};
use crate::packet::Packet;

const NOISE_PROTOCOL_NAME: &str = "Noise_NN_25519_ChaChaPoly_BLAKE2s";

type ChannelId = u8;

pub struct Protocol {
    inner: Arc<InnerProtocol>,
}

struct InnerProtocol {
    socket: UdpSocket,

    work_permit: Semaphore,
    waiting: Mutex<Vec<Waker>>,

    state: Mutex<MutableState>,
    flush: Notify,
    dead: AtomicBool,
    queue: Mutex<PacketQueue>,
}

struct MutableState {
    state: State,
    this_seq_pos: u64,
    other_seq_pos: u64,
    backlog: Vec<(u64, Packet)>,
    send_delay: Interval,

    last_recv: Instant,
    last_recv_wakeup: Pin<Box<Sleep>>,

    ping_timeout: Pin<Box<Sleep>>,

    read_interests: VecDeque<ReadInterest>,
}

enum State {
    Handshake { cipher: snow::HandshakeState },
}

enum Interest {
    Write(WriteInterest),
    Recv(ReadInterest),
    None,
}

struct WriteInterest {
    channel: ChannelId,
    data: BytesMut,
}

struct ReadInterest {
    channel: ChannelId,
    data: Bytes,
}

enum InterestResult {

}

impl InnerProtocol {
    fn poll_send_recv_loop(&self, cx: &mut Context, interest: Interest) -> Poll<std::io::Result<InterestResult>> {
        match self.poll_send_recv_loop_inner(cx, interest) {
            x @ Poll::Ready(Err(_)) => {
                self.dead.store(true, Ordering::Release);
                x
            }
            x => x
        }
    }

    fn poll_send_recv_loop_inner(&self, cx: &mut Context, interest: Interest) -> Poll<std::io::Result<InterestResult>> {
        let permit = if let Ok(permit) = self.work_permit.try_acquire() {
            permit
        } else {
            // TODO: this wont work
            self.waiting.lock().push(cx.waker().clone());
            return Poll::Pending;
        };

        let state = self.state.lock();

        let result = self.socket.poll_recv_ready(cx);


        permit.forget();
        Poll::Pending
    }

}

impl MutableState {
}

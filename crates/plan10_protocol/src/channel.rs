use crate::{ChannelId, Protocol};
use std::io::{Error, ErrorKind};
use std::pin::Pin;
use std::process::id;
use std::sync::atomic::Ordering;
use std::task::{Context, Poll};
use tokio::io::AsyncWrite;
use tokio::sync::mpsc::Sender;

pub struct Channel {
    id: ChannelId,
    protocol: Protocol,
}

impl AsyncWrite for Channel {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>> {
        if self.protocol.inner.dead.load(Ordering::Relaxed) {
            return Poll::Ready(Ok(0));
        }
        // locking this mutex is fine as it is exclusively used for these short locks
        self.protocol
            .inner
            .queue
            .lock()
            .enqueue(self.id, buf, cx.waker())
            .map(|x| Ok(x))
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        if self.protocol.inner.dead.load(Ordering::Relaxed) {
            Poll::Ready(Err(Error::from(ErrorKind::BrokenPipe)))
        } else {
            self.protocol.inner.flush.notify_one();
            Poll::Ready(Ok(()))
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        todo!()
    }
}

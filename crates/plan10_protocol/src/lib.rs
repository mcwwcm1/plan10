mod leb128;
mod packet;
mod sequence;

use crate::sequence::Sequence;
use std::sync::Arc;
use tokio::net::UdpSocket;

const NOISE_PROTOCOL_NAME: &str = "Noise_NN_25519_ChaChaPoly_BLAKE2s";

pub struct Protocol {
    inner: Arc<InnerProtocol>,
}

struct InnerProtocol {
    socket: UdpSocket,
    state: State,
    this_seq: Sequence,
    other_seq: Sequence,
    backup: [Option<Packet>; 256],
}

enum State {
    Handshake { cipher: snow::HandshakeState },
}

struct Packet {}

impl Protocol {}

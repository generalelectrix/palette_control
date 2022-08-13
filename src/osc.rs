use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket},
    sync::{
        mpsc::{channel, Sender},
        Arc,
    },
    thread,
};

use log::warn;

pub struct OscSender {
    send: Sender<(SocketAddr, Arc<Vec<u8>>)>,
}

impl OscSender {
    /// Initialize an OSC sender that will bind to the provided port.
    /// Spawns a thread that drains a queue of requests to send OSC packets.
    /// The packets should have been pre-encoded into OSC format.
    pub fn new(port: u16) -> Result<Self, Box<dyn Error>> {
        let host_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port);
        let sock = UdpSocket::bind(host_addr)?;
        let (send, recv) = channel::<(SocketAddr, Arc<Vec<u8>>)>();
        thread::spawn(move || {
            loop {
                let (dest_addr, packet) = match recv.recv() {
                    Ok(m) => m,
                    Err(_) => {
                        // Sender hung up, no more messages to send.
                        break;
                    }
                };
                if let Err(e) = sock.send_to(&packet, dest_addr) {
                    warn!("OSC send error: {}", e);
                }
            }
        });
        Ok(Self { send })
    }

    pub fn send(&self, addr: SocketAddr, packet: Arc<Vec<u8>>) {
        self.send.send((addr, packet));
    }
}

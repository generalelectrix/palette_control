use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::{error::Error, sync::mpsc::channel};

use client::Clients;
use control::Dispatcher;
use osc::OscSender;
use palette::Palette;
use shared::{ControlMessage, SubscriberConfig, SubscriberControlMessage};
use simple_error::bail;

mod client;
mod control;
mod osc;
mod palette;
mod subscriber;

fn main() -> Result<(), Box<dyn Error>> {
    let (send, recv) = channel();

    let dest_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 11000);

    send.send(ControlMessage::Subscriber(SubscriberControlMessage::Add(
        SubscriberConfig::Osc(SocketAddr::V4(dest_addr)),
    )))?;

    let clients = Clients::new(send)?;

    let mut dispatcher = Dispatcher::new(OscSender::new(10000)?, Palette::new(), clients);

    loop {
        let msg = match recv.recv() {
            Ok(msg) => msg,
            Err(_) => {
                bail!("Control message channel disconnected; exiting.");
            }
        };
        dispatcher.control(msg);
    }
}

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::thread;
use std::time::Duration;
use std::{error::Error, sync::mpsc::channel};

use client::Clients;
use control::Dispatcher;
use osc::OscSender;
use palette::Palette;
use shared::{
    Color, ControlMessage, PaletteControlMessage, SubscriberConfig, SubscriberControlMessage,
};
use simple_error::bail;

mod client;
mod control;
mod osc;
mod palette;
mod subscriber;

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::SimpleLogger::new().init().unwrap();
    let (send, recv) = channel();

    let dest_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 11000);

    send.send(ControlMessage::Subscriber(SubscriberControlMessage::Add(
        SubscriberConfig::Osc(SocketAddr::V4(dest_addr)),
    )))?;

    let clients = Clients::new(send.clone())?;

    let mut dispatcher = Dispatcher::new(OscSender::new(10000)?, Palette::new(), clients);

    // Test - periodically send a palette update.
    thread::spawn(move || {
        let mut colors = vec![
            Color {
                red: 1.,
                green: 1.,
                blue: 0.,
            },
            Color {
                red: 1.,
                green: 0.,
                blue: 1.,
            },
            Color {
                red: 0.,
                green: 1.,
                blue: 1.,
            },
        ];
        loop {
            if let Err(_) = send.send(ControlMessage::Palette(PaletteControlMessage::Set(
                colors.clone(),
            ))) {
                break;
            }
            thread::sleep(Duration::from_secs(5));
            colors.rotate_right(1);
        }
    });

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

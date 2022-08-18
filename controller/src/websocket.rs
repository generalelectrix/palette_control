use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
use reqwasm::websocket::{futures::WebSocket, Message};

use shared::ControlMessage;
use wasm_bindgen_futures::spawn_local;
use yew_agent::Dispatched;

use crate::event_bus::EventBus;

pub struct WebsocketService {
    pub tx: Sender<ControlMessage>,
}

impl WebsocketService {
    pub fn new() -> Self {
        let ws = WebSocket::open("ws://127.0.0.1:8081").unwrap();

        let (mut write, mut read) = ws.split();

        let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<ControlMessage>(1000);
        let mut event_bus = EventBus::dispatcher();

        spawn_local(async move {
            while let Some(msg) = in_rx.next().await {
                log::debug!("got event from channel! {:?}", msg);
                let s = serde_json::to_string(&msg).unwrap();
                write.send(Message::Text(s)).await.unwrap();
            }
        });

        spawn_local(async move {
            while let Some(msg) = read.next().await {
                let msg_contents = match msg {
                    Ok(Message::Text(data)) => data,
                    Ok(Message::Bytes(b)) => match String::from_utf8(b) {
                        Ok(val) => val,
                        Err(e) => {
                            log::error!("ws: {:?}", e);
                            continue;
                        }
                    },
                    Err(e) => {
                        log::error!("ws: {:?}", e);
                        continue;
                    }
                };
                match serde_json::from_str(&msg_contents) {
                    Ok(msg) => {
                        log::info!("Sending state change: {:?}", msg);
                        event_bus.send(msg);
                    }
                    Err(e) => {
                        log::error!("ws: {:?}", e);
                        continue;
                    }
                }
            }
            log::debug!("WebSocket Closed");
        });

        Self { tx: in_tx }
    }
}

use log::{error, info, warn};
use shared::{ControlMessage, StateChange};
use simple_error::bail;
use std::error::Error;
use std::net::TcpStream;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use websocket::server::upgrade::WsUpgrade;
use websocket::sync::server::upgrade::Buffer;
use websocket::sync::{Reader, Server, Writer};
use websocket::OwnedMessage;

type Senders = Arc<Mutex<Vec<Writer<TcpStream>>>>;

/// Handle client communication via websockets.
pub struct Clients {
    senders: Senders,
}

impl Clients {
    pub fn new(send: Sender<ControlMessage>) -> Result<Self, Box<dyn Error>> {
        let senders = Arc::new(Mutex::new(Vec::new()));
        let manager = Self {
            senders: senders.clone(),
        };
        // Create the websocket server and launch a thread to handle connections.
        let server = Server::bind("127.0.0.1:2794")?;

        thread::spawn(move || {
            for req in server.filter_map(Result::ok) {
                let (reader, writer) = match handle_upgrade_request(req) {
                    Ok(c) => c,
                    Err(e) => {
                        error!("Failed to handle websocket upgrade request: {}.", e);
                        continue;
                    }
                };
                // Add the writer to the collection of senders.
                // Use a new scope to immediately release the lock.
                {
                    senders.lock().unwrap().push(writer);
                }
                let send_copy = send.clone();
                // Spawn a thread to handle incoming requests.
                thread::spawn(move || {
                    handle_messages(reader, send_copy);
                });
            }
        });
        Ok(manager)
    }

    pub fn send_state_update(&self, msg: &StateChange) -> Result<(), Box<dyn Error>> {
        let serialized = OwnedMessage::Text(serde_json::to_string(&msg)?);
        for sender in self.senders.lock().unwrap().iter_mut() {
            if let Err(e) = sender.send_message(&serialized) {
                // FIXME: how do we know when a client has hung up?
                // Need to identify this condition and boot the sender from the
                // collection.
                error!("Websocket send error: {}.", e);
            }
        }
        Ok(())
    }
}

fn handle_upgrade_request(
    request: WsUpgrade<TcpStream, Option<Buffer>>,
) -> Result<(Reader<TcpStream>, Writer<TcpStream>), Box<dyn Error>> {
    if !request.protocols().contains(&"palette".to_string()) {
        request.reject().map_err(|(_, e)| e)?;
        bail!("Rejected websocket upgrade for incorrect protocol.")
    }

    let client = request
        .use_protocol("palette")
        .accept()
        .map_err(|(_, e)| e)?;

    Ok(client.split()?)
}

/// Handle incoming messages from a websocket reader, deserialize, and forward.
fn handle_messages(mut reader: Reader<TcpStream>, send: Sender<ControlMessage>) {
    for message_result in reader.incoming_messages() {
        // FIXME: will this terminate if the socket is closed?
        let message = match message_result {
            Ok(m) => m,
            Err(e) => {
                warn!("Websocket receive error: {}.", e);
                continue;
            }
        };
        let contents = match message {
            OwnedMessage::Text(t) => t,
            other => {
                warn!("Unhandled websocket message type: {:?}", other);
                continue;
            }
        };
        let control_msg: ControlMessage = match serde_json::from_str(&contents) {
            Ok(m) => m,
            Err(e) => {
                error!(
                    "Failed to deserialize websocket message: {}.\n\nMessage body:\n{}",
                    e, contents
                );
                continue;
            }
        };
        if send.send(control_msg).is_err() {
            info!("Terminating websocket receiver thread.");
            return;
        }
    }
}

use std::net::SocketAddr;

/// Maintain the collection of palette subscribers.
pub struct Subscribers {
    subs: Vec<Subscriber>,
    next_id: SubscriberId,
}

impl Subscribers {
    pub fn new() -> Self {
        Self {
            subs: Vec::new(),
            next_id: SubscriberId(0),
        }
    }

    pub fn control<E: EmitStateChange>(&mut self, msg: ControlMessage, emitter: &mut E) {
        match msg {
            ControlMessage::Add(cfg) => {
                let id = self.next_id;
                self.next_id.0 += 1;
                let sub = Subscriber { id, cfg };
                self.subs.push(sub.clone());
                emitter.emit_subscriber_state_change(StateChange::Added(sub));
            }
            ControlMessage::Remove(id) => {
                self.subs.retain(|sub| sub.id != id);
                emitter.emit_subscriber_state_change(StateChange::Removed(id));
            }
        }
    }
}

pub enum ControlMessage {
    Add(SubscriberConfig),
    Remove(SubscriberId),
}

pub enum StateChange {
    Added(Subscriber),
    Removed(SubscriberId),
}

pub trait EmitStateChange {
    fn emit_subscriber_state_change(&mut self, sc: StateChange);
}

/// A unique ID assigned to each subscriber when it is added.
/// Clients can refer to subscribers by this ID.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SubscriberId(u64);

#[derive(Debug, Clone)]
pub struct Subscriber {
    id: SubscriberId,
    cfg: SubscriberConfig,
}

#[derive(Debug, Clone)]
pub enum SubscriberConfig {
    Osc(SocketAddr),
}

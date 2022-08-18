use std::collections::HashMap;

use crate::color::Palette;
use shared::{
    Color, ControlMessage, PaletteStateChange, StateChange, SubscriberConfig, SubscriberId,
    SubscriberStateChange,
};
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::event_bus::EventBus;
use crate::websocket::WebsocketService;

pub enum Msg {
    HandleStateChange(StateChange),
    Send(ControlMessage),
}

pub struct App {
    palette: Vec<Color>,
    subscribers: HashMap<SubscriberId, SubscriberConfig>,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut wss = WebsocketService::new();

        wss.tx.try_send(shared::ControlMessage::Refresh).unwrap();

        Self {
            palette: vec![],
            subscribers: HashMap::new(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleStateChange)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        use StateChange::*;
        match msg {
            Msg::HandleStateChange(sc) => {
                match sc {
                    Palette(PaletteStateChange::Set(colors)) => {
                        self.palette = colors;
                    }
                    Subscriber(SubscriberStateChange::Added(sub)) => {
                        self.subscribers.insert(sub.id, sub.cfg);
                    }
                    Subscriber(SubscriberStateChange::Removed(id)) => {
                        self.subscribers.remove(&id);
                    }
                };
                true
            }
            Msg::Send(msg) => {
                self.wss.tx.try_send(msg).unwrap();
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let _ = ctx.link().callback(|_| Msg::SubmitMessage);

        html! {
            <Palette colors={self.palette.clone()} />
        }
    }
}

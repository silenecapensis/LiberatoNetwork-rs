/*
use libp2p::StreamProtocol;
use libp2p::swarm::{
    ConnectionHandler,
    ConnectionHandlerEvent,
    SubstreamProtocol,
};

pub struct MyHandler;

impl ConnectionHandler for MyHandler {
    type FromBehaviour = ();
    type ToBehaviour = ();
    type Error = std::io::Error;
    type InboundProtocol = StreamProtocol;
    type OutboundProtocol = StreamProtocol;
    type InboundOpenInfo = ();
    type OutboundOpenInfo = ();

    fn listen_protocol(&self) -> SubstreamProtocol<Self::InboundProtocol> {
        SubstreamProtocol::new(MY_PROTOCOL, ())
    }

    fn on_behaviour_event(&mut self, _event: Self::FromBehaviour) {}

    fn poll(
        &mut self,
        _cx: &mut std::task::Context,
    ) -> std::task::Poll<
        ConnectionHandlerEvent<
            Self::OutboundProtocol,
            Self::OutboundOpenInfo,
            Self::ToBehaviour,
            Self::Error,
        >
    > {
        std::task::Poll::Pending
    }
}
const MY_PROTOCOL: StreamProtocol = StreamProtocol::new("/muscarine-response/1.0.0");

#[derive(Debug)]
pub enum Message {
    Test,
}
    */
use libp2p::PeerId;
use libp2p::swarm::{
    DialPeerCondition,
    NetworkBehaviour,
    NetworkBehaviourAction,
    NotifyHandler,
    PollParameters,
};
use futures::task::{Context, Poll};


pub struct GmpcProto {
    /// `PeerId` of the local node.
    local_peer_id: PeerId,
}

impl GmpcProto {
    pub fn new(peer_id: PeerId) -> Self {
        GmpcProto {
            local_peer_id: peer_id,
        }
    }
}

impl NetworkBehaviour for GmpcProto {
    type ProtocolsHandler = ();
    type OutEvent = ();

    fn new_handler(&mut self) -> Self::ProtocolsHandler {
        unimplemented!()
    }

    fn addresses_of_peer(&mut self, peer_id: &PeerId) -> Vec<Multiaddr> {
        unimplemented!()
    }

    fn inject_connected(&mut self, peer_id: &PeerId) {
        unimplemented!()
    }

    fn inject_disconnected(&mut self, peer_id: &PeerId) {
        unimplemented!()
    }

    fn inject_event(&mut self, peer_id: PeerId, connection: ConnectionId, event: _) {
        unimplemented!()
    }

    fn poll(&mut self, cx: &mut Context<'a>, params: &mut impl PollParameters<SupportedProtocolsIter=_, ListenedAddressesIter=_, ExternalAddressesIter=_>) -> Poll<NetworkBehaviourAction<_, Self::OutEvent>> {
        unimplemented!()
    }
}
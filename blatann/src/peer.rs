use std::sync::{Arc, Mutex};

use blatann_event::{Publisher, Subscribable, Subscriber, SubscriberAction};
use uuid::Uuid;

use nrf_driver::ble_event::BleEventId;
use nrf_driver::common::consts::CONN_HANDLE_INVALID;
use nrf_driver::common::types::ConnHandle;
use nrf_driver::driver::NrfDriver;
use nrf_driver::driver_events::NrfEventPublisher;
use nrf_driver::gap::enums::{BleGapPhy, BleGapRole};
use nrf_driver::gap::events::GapEventDisconnected;
use nrf_driver::gap::types::{BleGapAddress, BleGapConnParams};

use crate::events::{ConnectionEvent, DisconnectionEvent};

pub type PeerRole = BleGapRole;
pub type Phy = BleGapPhy;

pub enum PeerState {
    Disconnected,
    Connecting,
    Connected,
}


struct State {
    conn_handle: ConnHandle,
    peer_address: Option<BleGapAddress>,
    connection_state: PeerState,
    conn_params: BleGapConnParams,
    mtu_size: u32,
    preferred_mtu_size: u32,
    negotiated_mtu_size: Option<usize>,
    preferred_phy: Phy,
    current_phy: Phy,
    disconnection_reason: u32,
    connection_based_subs: Vec<(BleEventId, Uuid)>,
}

impl State {
    fn new(connection_state: PeerState, conn_params: &BleGapConnParams) -> Self {
        Self {
            conn_handle: CONN_HANDLE_INVALID,
            peer_address: None,
            connection_state,
            conn_params: conn_params.clone(),
            mtu_size: 23,
            preferred_mtu_size: 23,
            negotiated_mtu_size: None,
            preferred_phy: Phy::Auto,
            current_phy: Phy::OneMbps,
            disconnection_reason: 0,
            connection_based_subs: vec![],
        }
    }
}

pub struct Peer {
    role: PeerRole,
    max_mtu_size: usize,
    state: Mutex<State>,
    driver: Arc<NrfDriver>,

    pub on_connect: Publisher<Self, ConnectionEvent>,
    pub on_disconnect: Publisher<Self, DisconnectionEvent>,
}

impl Peer {
    pub(crate) fn new(driver: &Arc<NrfDriver>, role: PeerRole, conn_params: &BleGapConnParams) -> Arc<Self> {
        let init_conn_state = match role {
            BleGapRole::Invalid => panic!("Shouldn't use this!"),
            BleGapRole::Peripheral => PeerState::Disconnected,
            BleGapRole::Central => PeerState::Connecting,
        };

        let peer = Arc::new(Self {
            role,
            max_mtu_size: 23,  // TODO magic number
            state: Mutex::new(State::new(init_conn_state, conn_params)),
            driver: driver.clone(),

            on_connect: Publisher::new("On Connect"),
            on_disconnect: Publisher::new("On Disconnect"),
        });

        driver.events.disconnected.subscribe(peer.clone());

        return peer;
    }

    pub(crate) fn peer_connected(self: &Arc<Self>, conn_handle: ConnHandle, address: &BleGapAddress, conn_params: &BleGapConnParams) {
        {
            let mut state = self.state.lock().unwrap();
            state.connection_state = PeerState::Connected;
            state.conn_handle = conn_handle;
            state.peer_address = Some(address.clone());
            state.conn_params = conn_params.clone();
            state.negotiated_mtu_size = None;
            state.mtu_size = 23; // TODO magic number

            // disconnect from all the connection-based event handlers
            for (event_id, sub_id) in state.connection_based_subs {
                self.driver.unsubscribe_from_event(event_id, sub_id)
            }
            state.connection_based_subs.clear()
        }

        self.on_connect.dispatch(self.clone(), ConnectionEvent {})
    }

    pub(crate) fn subscribe_for_connection<S: 'static, E: Clone>(self: &Arc<Self>, subscriber: Arc<S>, event: &NrfEventPublisher<E>)
        where S: Subscriber<NrfDriver, E> {
        let event_id = event.id();
        let subscription_id = event.subscribe(subscriber);
        let mut state = self.state.lock().unwrap();
        state.connection_based_subs.push((event_id, subscription_id))
    }
}

impl Subscriber<NrfDriver, GapEventDisconnected> for Peer {
    fn handle(self: Arc<Self>, _sender: Arc<NrfDriver>, event: GapEventDisconnected) -> Option<SubscriberAction> {
        let mut send_event = false;
        {
            let mut state = self.state.lock().unwrap();
            if state.conn_handle == event.conn_handle {
                state.connection_state = PeerState::Disconnected;
                state.conn_handle = CONN_HANDLE_INVALID;
                send_event = true;

                // disconnect from all the connection-based event handlers
                for (event_id, sub_id) in state.connection_based_subs {
                    self.driver.unsubscribe_from_event(event_id, sub_id)
                }
                state.connection_based_subs.clear()
            }
        }

        if send_event {
            self.on_disconnect.dispatch(self.clone(), DisconnectionEvent { reason: event.reason });
        }

        // TODO: Unsubscribe from everything
        return None;
    }
}

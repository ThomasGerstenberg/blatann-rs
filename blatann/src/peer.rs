use std::sync::{Arc, Mutex, MutexGuard};

use blatann_event::{EventWaitable, Publisher, Subscribable, Subscriber, SubscriberAction};
use uuid::Uuid;

use nrf_driver::ble_event::{BleEventDataType, BleEventId};
use nrf_driver::common::consts::CONN_HANDLE_INVALID;
use nrf_driver::common::types::ConnHandle;
use nrf_driver::driver::NrfDriver;
use nrf_driver::driver_events::NrfEventPublisher;
use nrf_driver::error::NrfResult;
use nrf_driver::gap::enums::{BleGapPhy, BleGapRole};
use nrf_driver::gap::events::{
    GapEventDataLengthUpdate, GapEventDataLengthUpdateRequest, GapEventDisconnected,
    GapEventPhyUpdate, GapEventPhyUpdateRequest,
};
use nrf_driver::gap::types::{BleGapAddress, BleGapConnParams};

use crate::consts::MTU_SIZE_DEFAULT;
use crate::events::*;

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
            mtu_size: MTU_SIZE_DEFAULT,
            preferred_mtu_size: MTU_SIZE_DEFAULT,
            negotiated_mtu_size: None,
            preferred_phy: Phy::AUTO,
            current_phy: Phy::ONE_MBPS,
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
    pub on_phy_updated: Publisher<Self, PhyUpdateEvent>,
    pub on_data_length_updated: Publisher<Self, DataLengthUpdateEvent>,
}

impl Peer {
    pub(crate) fn new(
        driver: &Arc<NrfDriver>,
        role: PeerRole,
        conn_params: &BleGapConnParams,
    ) -> Arc<Self> {
        let init_conn_state = match role {
            BleGapRole::Invalid => panic!("Shouldn't use this!"),
            BleGapRole::Peripheral => PeerState::Disconnected,
            BleGapRole::Central => PeerState::Connecting,
        };

        let peer = Arc::new(Self {
            role,
            max_mtu_size: 23, // TODO magic number
            state: Mutex::new(State::new(init_conn_state, conn_params)),
            driver: driver.clone(),

            on_connect: Publisher::new("On Connect"),
            on_disconnect: Publisher::new("On Disconnect"),
            on_phy_updated: Publisher::new("On Phy Update"),
            on_data_length_updated: Publisher::new("On Data Length Update"),
        });

        driver.events.disconnected.subscribe(peer.clone());

        return peer;
    }

    pub fn disconnect(self: &Arc<Self>) -> NrfResult<Arc<EventWaitable<Self, DisconnectionEvent>>> {
        let conn_handle = { self.state.lock().unwrap().conn_handle };

        self.driver
            .ble_gap_disconnect(conn_handle)
            .and_then(|_| Ok(EventWaitable::new(&self.on_disconnect)))
    }

    pub(crate) fn peer_connected(
        self: &Arc<Self>,
        conn_handle: ConnHandle,
        address: &BleGapAddress,
        conn_params: &BleGapConnParams,
    ) {
        self.update_state(|state| {
            state.connection_state = PeerState::Connected;
            state.conn_handle = conn_handle;
            state.peer_address = Some(address.clone());
            state.conn_params = conn_params.clone();
            state.negotiated_mtu_size = None;
            state.mtu_size = 23; // TODO magic number

            // disconnect from all the connection-based event handlers
            for (event_id, sub_id) in state.connection_based_subs.iter() {
                self.driver.unsubscribe_from_event(*event_id, *sub_id)
            }
            state.connection_based_subs.clear();
        });

        self.subscribe_for_connection(self.clone(), &self.driver.events.phy_update_request);
        self.subscribe_for_connection(self.clone(), &self.driver.events.phy_update);
        self.subscribe_for_connection(self.clone(), &self.driver.events.data_length_update_request);
        self.subscribe_for_connection(self.clone(), &self.driver.events.data_length_update);

        self.on_connect.dispatch(self.clone(), ConnectionEvent {})
    }

    pub(crate) fn subscribe_for_connection<S: 'static, E: BleEventDataType>(
        self: &Arc<Self>,
        subscriber: Arc<S>,
        event: &NrfEventPublisher<E>,
    ) where
        S: Subscriber<NrfDriver, E>,
    {
        let event_id = event.id();
        let subscription_id = event.subscribe(subscriber);

        self.update_state(|s| s.connection_based_subs.push((event_id, subscription_id)));
    }

    fn conn_handle(&self) -> ConnHandle {
        let state = self.state.lock().unwrap();
        state.conn_handle
    }

    fn read_state<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&MutexGuard<State>) -> T,
    {
        let state = self.state.lock().unwrap();
        f(&state)
    }

    fn update_state<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut MutexGuard<State>) -> T,
    {
        let mut state = self.state.lock().unwrap();
        f(&mut state)
    }

    fn read_state_if<F, T>(&self, conn_handle: ConnHandle, f: F) -> Option<T>
    where
        F: FnOnce(&MutexGuard<State>) -> T,
    {
        let state = self.state.lock().unwrap();
        if conn_handle == state.conn_handle {
            Some(f(&state))
        } else {
            None
        }
    }

    fn update_state_if<F, T>(&self, conn_handle: ConnHandle, f: F) -> Option<T>
    where
        F: FnOnce(&mut MutexGuard<State>) -> T,
    {
        let mut state = self.state.lock().unwrap();
        if conn_handle == state.conn_handle {
            Some(f(&mut state))
        } else {
            None
        }
    }
}

impl Subscriber<NrfDriver, GapEventDisconnected> for Peer {
    fn handle(
        self: Arc<Self>,
        _sender: Arc<NrfDriver>,
        event: GapEventDisconnected,
    ) -> Option<SubscriberAction> {
        self.update_state_if(event.conn_handle, |s| {
            s.connection_state = PeerState::Disconnected;
            s.conn_handle = CONN_HANDLE_INVALID;

            // disconnect from all the connection-based event handlers
            for (event_id, sub_id) in s.connection_based_subs.iter() {
                self.driver.unsubscribe_from_event(*event_id, *sub_id);
            }
        })?;

        self.on_disconnect.dispatch(
            self.clone(),
            DisconnectionEvent {
                reason: event.reason,
            },
        );

        return None;
    }
}

impl Subscriber<NrfDriver, GapEventPhyUpdateRequest> for Peer {
    fn handle(
        self: Arc<Self>,
        sender: Arc<NrfDriver>,
        event: GapEventPhyUpdateRequest,
    ) -> Option<SubscriberAction> {
        let (conn_handle, preferred_phy) =
            self.read_state_if(event.conn_handle, |s| (s.conn_handle, s.preferred_phy))?;

        debug!(
            "Peer-preferred phy - rx:{:?}, tx:{:?}, ours - {:?}",
            event.peer_preferred_phys.rx_phys, event.peer_preferred_phys.tx_phys, preferred_phy,
        );

        sender
            .ble_gap_phy_update(conn_handle, preferred_phy, preferred_phy)
            .unwrap();

        return None;
    }
}

impl Subscriber<NrfDriver, GapEventPhyUpdate> for Peer {
    fn handle(
        self: Arc<Self>,
        _sender: Arc<NrfDriver>,
        event: GapEventPhyUpdate,
    ) -> Option<SubscriberAction> {
        self.update_state_if(event.conn_handle, |s| {
            s.current_phy = event.tx_phy;
        })?;

        self.on_phy_updated.dispatch(
            self.clone(),
            PhyUpdateEvent {
                tx_phy: event.tx_phy,
                rx_phy: event.rx_phy,
            },
        );
        return None;
    }
}

impl Subscriber<NrfDriver, GapEventDataLengthUpdateRequest> for Peer {
    fn handle(
        self: Arc<Self>,
        sender: Arc<NrfDriver>,
        event: GapEventDataLengthUpdateRequest,
    ) -> Option<SubscriberAction> {
        if self.conn_handle() == event.conn_handle {
            sender
                .ble_gap_data_length_update(event.conn_handle, None)
                .unwrap();
        }

        None
    }
}

impl Subscriber<NrfDriver, GapEventDataLengthUpdate> for Peer {
    fn handle(
        self: Arc<Self>,
        _sender: Arc<NrfDriver>,
        event: GapEventDataLengthUpdate,
    ) -> Option<SubscriberAction> {
        if self.conn_handle() == event.conn_handle {
            let params = DataLengthUpdateEvent {
                tx_bytes: event.effective_params.max_tx_octets,
                rx_bytes: event.effective_params.max_rx_octets,
                tx_time_us: event.effective_params.max_tx_time_us,
                rx_time_us: event.effective_params.max_rx_time_us,
            };

            self.on_data_length_updated.dispatch(self.clone(), params);
        }

        None
    }
}

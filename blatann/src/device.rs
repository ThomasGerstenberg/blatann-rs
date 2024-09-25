use std::sync::{Arc, Mutex};

use nrf_driver::driver::NrfDriver;
use nrf_driver::error::NrfError;
use nrf_driver::DRIVER_MANAGER;

use crate::advertiser::Advertiser;
use crate::peer::{Peer, PeerRole};
use blatann_event::{Subscribable, Subscriber, SubscriberAction};
use nrf_driver::common::events::CommonEventMemRequest;
use nrf_driver::gap::enums::BleGapRole;
use nrf_driver::gap::events::{GapEventConnected, GapEventDisconnected};
use nrf_driver::gap::types::BleGapConnParams;

struct State {
    default_conn_params: BleGapConnParams,
}

impl State {
    fn new(conn_params: BleGapConnParams) -> Self {
        Self {
            default_conn_params: conn_params,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new(BleGapConnParams::new(15_f64, 30_f64, 4000_f64, 0))
    }
}

pub struct BleDevice {
    port: String,
    driver: Arc<NrfDriver>,
    state: Mutex<State>,
    pub advertiser: Arc<Advertiser>,
    pub central: Arc<Peer>,
}

impl BleDevice {
    pub fn new(port: String, baud: u32) -> Arc<Self> {
        let driver = {
            let mut manager = DRIVER_MANAGER.lock().unwrap();
            manager.create(port.clone(), baud, true)
        };
        let state: State = Default::default();
        let central = Peer::new(&driver, PeerRole::Peripheral, &state.default_conn_params);
        let advertiser = Advertiser::new(&driver, &central);

        let device = Arc::new(Self {
            port,
            advertiser,
            driver: driver.clone(),
            central: central.clone(),
            state: Mutex::new(state),
        });
        driver.events.connected.subscribe(device.clone());
        driver.events.disconnected.subscribe(device.clone());

        return device;
    }

    pub fn open(&self) -> Result<(), NrfError> {
        self.driver.open().and_then(|_| self.driver.ble_enable())
    }
}

impl Drop for BleDevice {
    fn drop(&mut self) {
        // Remove the driver then close it
        DRIVER_MANAGER.lock().unwrap().remove(&self.driver.port);
        self.driver.close();
    }
}

impl Subscriber<NrfDriver, CommonEventMemRequest> for BleDevice {
    fn handle(
        self: Arc<Self>,
        sender: Arc<NrfDriver>,
        event: CommonEventMemRequest,
    ) -> Option<SubscriberAction> {
        sender
            .ble_user_mem_reply(event.conn_handle)
            .unwrap_or_else(|e| {
                error!("ble_user_mem_reply got error {:?}", e);
            });
        return None;
    }
}

impl Subscriber<NrfDriver, GapEventConnected> for BleDevice {
    fn handle(
        self: Arc<Self>,
        _sender: Arc<NrfDriver>,
        event: GapEventConnected,
    ) -> Option<SubscriberAction> {
        if let BleGapRole::Peripheral = event.role {
            info!("Peer connected!");
            self.central
                .peer_connected(event.conn_handle, &event.address, &event.conn_params);
        }
        return None;
    }
}

impl Subscriber<NrfDriver, GapEventDisconnected> for BleDevice {
    fn handle(
        self: Arc<Self>,
        _sender: Arc<NrfDriver>,
        _event: GapEventDisconnected,
    ) -> Option<SubscriberAction> {
        // TODO
        return None;
    }
}

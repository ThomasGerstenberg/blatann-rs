use std::sync::atomic::{Ordering, AtomicBool};
use std::sync::{Mutex, Arc};

use crate::ffi;
use crate::ble_event::BleEvent;
use crate::error::NrfError;
use crate::event_publisher::EventPublisher;
use crate::common::events::*;
use crate::gap::events::*;
use crate::gap::types::*;
use crate::manager::event_handler;


#[allow(dead_code)]
pub struct NrfDriver {
    pub port: String,
    pub id: usize,
    pub events: NrfDriverEvents,
    adapter: Mutex<*mut ffi::adapter_t>,
    link_layer: Mutex<*mut ffi::data_link_layer_t>,
    transport_layer: Mutex<*mut ffi::transport_layer_t>,
    log_driver_comms: bool,
    is_open: AtomicBool,
}

#[allow(dead_code)]
pub struct NrfDriverEvents {
    pub gap_timeout: EventPublisher<NrfDriver, BleGapTimeout>
}

impl NrfDriver {
    pub(crate) fn new(port: String, baud: u32, log_driver_comms: bool) -> Self {
        unsafe {
            let phy_layer = ffi::sd_rpc_physical_layer_create_uart(
                port.as_ptr() as *const _,
                baud,
                ffi::sd_rpc_flow_control_t_SD_RPC_FLOW_CONTROL_NONE,
                ffi::sd_rpc_parity_t_SD_RPC_PARITY_NONE,
            );
            let link_layer = ffi::sd_rpc_data_link_layer_create_bt_three_wire(phy_layer, 100);
            let transport_layer = ffi::sd_rpc_transport_layer_create(link_layer, 100);
            let rpc_adapter = ffi::sd_rpc_adapter_create(transport_layer);
            let id = (*rpc_adapter).internal as usize;
            Self {
                port,
                id,
                adapter: Mutex::new(rpc_adapter),
                link_layer: Mutex::new(link_layer),
                transport_layer: Mutex::new(transport_layer),
                log_driver_comms,
                is_open: AtomicBool::new(false),
                events: NrfDriverEvents {
                    gap_timeout: EventPublisher::new()
                }
            }
        }
    }

    pub fn open(&self) -> Result<(), NrfError> {
        if self.is_open.load(Ordering::Relaxed) {
            return Ok(());
        }

        let err = unsafe {
            let adapter = self.adapter.lock().unwrap();
            ffi::sd_rpc_open(*adapter, None, Some(event_handler), None)
        };

        if err == ffi::NRF_SUCCESS {
            self.is_open.store(true, Ordering::Relaxed);
            Ok(())
        } else {
            Err(NrfError::new(err))
        }
    }

    pub fn close(&self) {
        if !self.is_open.load(Ordering::Relaxed) {
            return;
        }
        self.is_open.store(false, Ordering::Relaxed);
        unsafe {
            let adapter = self.adapter.lock().unwrap();
            ffi::sd_rpc_conn_reset(*adapter, ffi::sd_rpc_reset_t_SYS_RESET);
            ffi::sd_rpc_close(*adapter);
        }
    }

    pub fn ble_enable(&self) -> Result<(), NrfError> {
        let mut ram_base = 0u32;
        let _ram_base_ptr: *mut u32 = &mut ram_base;
        let err = unsafe {
            let adapter = self.adapter.lock().unwrap();
            ffi::sd_ble_enable(*adapter, _ram_base_ptr)
        };

        NrfError::make_result(err)
    }

    pub fn ble_gap_addr_get(&self) -> Result<BleGapAddress, NrfError> {
        let mut addr = ffi::ble_gap_addr_t {
            _bitfield_1: ffi::ble_gap_addr_t::new_bitfield_1(0, 0),
            addr: [0; 6],
        };

        let err = unsafe {
            let adapter = self.adapter.lock().unwrap();
            ffi::sd_ble_gap_addr_get(*adapter, &mut addr)
        };

        return NrfError::make_result_typed(err, || BleGapAddress::new_from_c(&addr));
    }

    pub fn ble_gap_addr_set(&self, address: &BleGapAddress) -> Result<(), NrfError> {
        let addr = address.to_c();

        let err = unsafe {
            let adapter = self.adapter.lock().unwrap();
            ffi::sd_ble_gap_addr_set(*adapter, &addr)
        };

        NrfError::make_result(err)
    }

    pub fn ble_gap_adv_start(&self, params: &BleGapAdvParams) -> Result<(), NrfError> {
        let params = params.to_c();

        let err = unsafe {
            let adapter = self.adapter.lock().unwrap();
            ffi::sd_ble_gap_adv_start(*adapter, &params, 0)
        };

        NrfError::make_result(err)
    }

    pub fn ble_gap_adv_stop(&self) -> Result<(), NrfError> {
        let err = unsafe {
            let adapter = self.adapter.lock().unwrap();
            ffi::sd_ble_gap_adv_stop(*adapter)
        };

        NrfError::make_result(err)
    }

    pub(crate) fn process_event(self: Arc<Self>, ble_event: &BleEvent) {
        match ble_event {
            BleEvent::Common(sub_event) => {
                match sub_event {
                    BleCommonEvent::MemRequest(_) => {}
                    BleCommonEvent::MemRelease(_) => {}
                }
            }
            BleEvent::Gap(sub_event) => {
                match sub_event {
                    BleGapEvent::Timeout(e) => {
                        self.events.gap_timeout.dispatch(self.clone(), e.clone())
                    }
                }
            }
        }
    }
}


impl Drop for NrfDriver {
    fn drop(&mut self) {
        self.close();
        unsafe {
            let adapter = self.adapter.lock().unwrap();
            ffi::sd_rpc_adapter_delete(*adapter);
        }
    }
}

unsafe impl Send for NrfDriver {}
unsafe impl Sync for NrfDriver {}
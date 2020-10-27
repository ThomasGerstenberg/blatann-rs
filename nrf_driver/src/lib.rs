#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate lazy_static;

pub mod gap;
pub mod nrf_error;
pub mod events;
pub mod common;
mod utils;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod driver;  // Auto-genned C bindings

use std::sync::{Mutex, Arc};
use crate::gap::types::*;
use crate::nrf_error::NrfError;
use std::sync::atomic::{AtomicBool, Ordering};


#[allow(dead_code)]
#[derive(Debug)]
pub struct NrfDriver {
    pub(crate) adapter: Mutex<*mut driver::adapter_t>,
    link_layer: Mutex<*mut driver::data_link_layer_t>,
    transport_layer: Mutex<*mut driver::transport_layer_t>,
    log_driver_comms: bool,
    is_open: AtomicBool,
    port: String,
}

impl NrfDriver {
    pub(crate) fn new(port: String, baud: u32, log_driver_comms: bool) -> Self {
        unsafe {
            let phy_layer = driver::sd_rpc_physical_layer_create_uart(
                port.as_ptr() as *const _,
                baud,
                driver::sd_rpc_flow_control_t_SD_RPC_FLOW_CONTROL_NONE,
                driver::sd_rpc_parity_t_SD_RPC_PARITY_NONE,
            );
            let link_layer = driver::sd_rpc_data_link_layer_create_bt_three_wire(phy_layer, 100);
            let transport_layer = driver::sd_rpc_transport_layer_create(link_layer, 100);
            let rpc_adapter = driver::sd_rpc_adapter_create(transport_layer);

            Self {
                adapter: Mutex::new(rpc_adapter),
                link_layer: Mutex::new(link_layer),
                transport_layer: Mutex::new(transport_layer),
                log_driver_comms,
                is_open: AtomicBool::new(false),
                port,
            }
        }
    }

    pub fn open(&self) -> Result<(), NrfError> {
        if self.is_open.load(Ordering::Relaxed) {
            return Ok(());
        }

        let err = unsafe {
            let adapter = self.adapter.lock().unwrap();
            driver::sd_rpc_open(*adapter, None, Some(event_handler), None)
        };

        if err == driver::NRF_SUCCESS {
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
            driver::sd_rpc_conn_reset(*adapter, driver::sd_rpc_reset_t_SYS_RESET);
            driver::sd_rpc_close(*adapter);
        }
    }

    pub fn ble_enable(&self) -> Result<(), NrfError> {
        let mut ram_base = 0u32;
        let _ram_base_ptr: *mut u32 = &mut ram_base;
        let err = unsafe {
            let adapter = self.adapter.lock().unwrap();
            driver::sd_ble_enable(*adapter, _ram_base_ptr)
        };

        NrfError::make_result(err)
    }

    pub fn ble_gap_addr_get(&self) -> Result<BleGapAddress, NrfError> {
        let mut addr = driver::ble_gap_addr_t {
            _bitfield_1: driver::ble_gap_addr_t::new_bitfield_1(0, 0),
            addr: [0; 6],
        };

        let err = unsafe {
            let adapter = self.adapter.lock().unwrap();
            driver::sd_ble_gap_addr_get(*adapter, &mut addr)
        };

        return NrfError::make_result_typed(err, || BleGapAddress::new_from_c(&addr));
    }

    pub fn ble_gap_addr_set(&self, address: &BleGapAddress) -> Result<(), NrfError> {
        let addr = address.to_c();

        let err = unsafe {
            let adapter = self.adapter.lock().unwrap();
            driver::sd_ble_gap_addr_set(*adapter, &addr)
        };

        NrfError::make_result(err)
    }

    pub fn ble_gap_adv_start(&self, params: BleGapAdvParams) -> Result<(), NrfError> {
        let params = params.to_c();

        let err = unsafe {
            let adapter = self.adapter.lock().unwrap();
            driver::sd_ble_gap_adv_start(*adapter, &params, 0)
        };

        NrfError::make_result(err)
    }

    pub fn ble_gap_adv_stop(&self) -> Result<(), NrfError> {
        let err = unsafe {
            let adapter = self.adapter.lock().unwrap();
            driver::sd_ble_gap_adv_stop(*adapter)
        };

        NrfError::make_result(err)
    }
}


impl Drop for NrfDriver {
    fn drop(&mut self) {
        self.close();
        unsafe {
            let adapter = self.adapter.lock().unwrap();
            driver::sd_rpc_adapter_delete(*adapter);
        }
    }
}

unsafe impl Send for NrfDriver {}
unsafe impl Sync for NrfDriver {}

pub struct NrfDriverManager {
    drivers: Mutex<Vec<Arc<NrfDriver>>>
}

impl NrfDriverManager {
    pub fn new() -> Self {
        Self {
            drivers: Mutex::new(vec![])
        }
    }

    pub fn create(&mut self, port: String, baud: u32, log_driver_comms: bool) -> Arc<NrfDriver> {
        let mut drivers = self.drivers.lock().unwrap();
        let driver = Arc::new(NrfDriver::new(port, baud, log_driver_comms));
        drivers.push(Arc::clone(&driver));

        driver
    }

    pub fn remove(&mut self, driver: Arc<NrfDriver>) {
        let mut drivers = self.drivers.lock().unwrap();
        drivers.retain(|x| *x.port != driver.port)
    }

    pub(crate) fn find_by_adapter(&self, adapter: *mut driver::adapter_t) -> Option<Arc<NrfDriver>> {
        let drivers = self.drivers.lock().unwrap();
        for d in &*drivers {
            let a = *d.adapter.lock().unwrap();
            if unsafe {std::ptr::eq((*adapter).internal, (*a).internal)} {
                return Some(d.clone())
            }
        }
        return None
    }
}


lazy_static! {
    pub static ref DRIVER_MANAGER: Mutex<NrfDriverManager> = Mutex::new(NrfDriverManager::new());
}


#[no_mangle]
unsafe extern "C" fn event_handler(adapter: *mut driver::adapter_t, _ble_event: *mut driver::ble_evt_t) {
    let driver = match DRIVER_MANAGER.lock().unwrap().find_by_adapter(adapter) {
        None => return,
        Some(x) => x,
    };

    println!("Got event for {}", driver.port);
}

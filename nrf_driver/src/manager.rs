use num_traits::FromPrimitive;
use std::ffi::CStr;
use std::sync::{mpsc, Arc, Mutex, Weak};
use std::thread;

use crate::ble_event::BleEvent;
use crate::common::enums::{BleLogSeverity, RpcAppStatus};
use crate::driver::NrfDriver;
use crate::ffi;
use crate::ffi::{sd_rpc_app_status_t, sd_rpc_log_severity_t};
use crate::DRIVER_MANAGER;

pub struct NrfDriverThreadCoordinator {
    driver: Weak<NrfDriver>,
    sender: mpsc::Sender<BleEvent>,
}

pub struct NrfDriverManager {
    coordinators: Mutex<Vec<NrfDriverThreadCoordinator>>,
}

impl NrfDriverManager {
    pub fn new() -> Self {
        Self {
            coordinators: Mutex::new(vec![]),
        }
    }

    pub fn create(&mut self, port: String, baud: u32, log_driver_comms: bool) -> Arc<NrfDriver> {
        // Create the driver
        let driver = Arc::new(NrfDriver::new(port.clone(), baud, log_driver_comms));
        // Setup the channels for handling events
        let (sender, receiver) = mpsc::channel();

        // Add the coordinator to the list
        let mut coordinators = self.coordinators.lock().unwrap();
        coordinators.push(NrfDriverThreadCoordinator {
            driver: Arc::downgrade(&driver),
            sender,
        });

        // Star the event thread
        let thread_driver = Arc::clone(&driver);
        thread::Builder::new()
            .name(format!("{}_Thread", port))
            .spawn(move || run_event_loop(thread_driver, receiver))
            .unwrap();

        return Arc::clone(&driver);
    }

    pub fn remove(&mut self, port: &str) {
        let mut coordinators = self.coordinators.lock().unwrap();
        coordinators.retain(|x| match x.driver.upgrade() {
            Some(d) => d.port != port,
            None => false,
        });
    }

    pub(crate) fn find_by_adapter(
        &self,
        adapter: *mut ffi::adapter_t,
    ) -> Option<NrfDriverThreadCoordinator> {
        let adapter_id = unsafe { (*adapter).internal as usize };

        let mut entries = self.coordinators.lock().unwrap();
        let mut cleanup = false;

        for entry in &*entries {
            if let Some(driver) = entry.driver.upgrade() {
                if driver.id == adapter_id {
                    return Some(NrfDriverThreadCoordinator {
                        driver: Arc::downgrade(&driver),
                        sender: entry.sender.clone(),
                    });
                }
            } else {
                cleanup = true;
            }
        }

        if cleanup {
            entries.retain(|ref c| match c.driver.upgrade() {
                Some(_) => true,
                None => false,
            })
        }
        return None;
    }
}

fn run_event_loop(driver: Arc<NrfDriver>, receiver: mpsc::Receiver<BleEvent>) {
    loop {
        let ble_event = match receiver.recv() {
            Ok(e) => e,
            Err(_) => return,
        };

        driver.clone().process_event(ble_event);
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn event_handler(
    adapter: *mut ffi::adapter_t,
    ble_event: *mut ffi::ble_evt_t,
) {
    let manager = DRIVER_MANAGER.lock().unwrap();
    let coordinator = match manager.find_by_adapter(adapter) {
        None => return,
        Some(x) => x,
    };

    let event = BleEvent::from_c(ble_event);
    coordinator.sender.send(event).unwrap();
}

#[no_mangle]
pub(crate) unsafe extern "C" fn log_handler(
    _adapter: *mut ffi::adapter_t,
    severity: sd_rpc_log_severity_t,
    log_message: *const ::std::os::raw::c_char,
) {
    let c_str = CStr::from_ptr(log_message);
    let severity = BleLogSeverity::from_u32(severity as u32).unwrap_or(BleLogSeverity::Info);
    log!(severity.into(), "{}", c_str.to_str().unwrap().to_owned());
}

#[no_mangle]
pub(crate) unsafe extern "C" fn status_handler(
    _adapter: *mut ffi::adapter_t,
    code: sd_rpc_app_status_t,
    message: *const ::std::os::raw::c_char,
) {
    let c_str = CStr::from_ptr(message);
    let status = RpcAppStatus::from_u32(code as u32).unwrap_or(RpcAppStatus::Unknown);
    debug!("{:?}: {}", status, c_str.to_str().unwrap().to_owned())
}

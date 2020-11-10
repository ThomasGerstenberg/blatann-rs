use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use crate::ffi;
use crate::ble_event::BleEvent;
use crate::driver::NrfDriver;
use crate::DRIVER_MANAGER;

pub struct NrfDriverThreadCoordinator {
    driver: Arc<NrfDriver>,
    sender: mpsc::Sender<BleEvent>,
}

pub struct NrfDriverManager {
    coordinators: Mutex<Vec<NrfDriverThreadCoordinator>>
}

impl NrfDriverManager {
    pub fn new() -> Self {
        Self {
            coordinators: Mutex::new(vec![])
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
            driver: Arc::clone(&driver),
            sender
        });

        // Star the event thread
        let thread_driver = Arc::clone(&driver);
        thread::Builder::new().name(format!("{}_Thread", port)).spawn(move || {
            run_event_loop(thread_driver, receiver)
        }).unwrap();

        return Arc::clone(&driver);
    }

    pub fn remove(&mut self, driver: Arc<NrfDriver>) {
        let mut coordinators = self.coordinators.lock().unwrap();
        coordinators.retain(|x| {
            x.driver.port != driver.port
        });
    }

    pub(crate) fn find_by_adapter(&self, adapter: *mut ffi::adapter_t) -> Option<NrfDriverThreadCoordinator> {
        let adapter_id = unsafe { (*adapter).internal as usize };

        let entries = self.coordinators.lock().unwrap();

        for entry in &*entries {
            if entry.driver.id == adapter_id {
                return Some(NrfDriverThreadCoordinator {
                    driver: Arc::clone(&entry.driver),
                    sender: entry.sender.clone()
                });
            }
        }
        return None
    }
}


fn run_event_loop(driver: Arc<NrfDriver>, receiver: mpsc::Receiver<BleEvent>) {
    loop {
        let ble_event = match receiver.recv() {
            Ok(e) => e,
            Err(_) => return
        };

        driver.clone().process_event(&ble_event);
    }
}


#[no_mangle]
pub(crate) unsafe extern "C" fn event_handler(adapter: *mut ffi::adapter_t, ble_event: *mut ffi::ble_evt_t) {
    let manager = DRIVER_MANAGER.lock().unwrap();
    let coordinator = match manager.find_by_adapter(adapter) {
        None => return,
        Some(x) => x,
    };

    match BleEvent::from_c(ble_event) {
        None => {
            warn!("Unable to decode event, id {}", (*ble_event).header.evt_id);
        }
        Some(event) => {
            coordinator.sender.send(event).unwrap();
        }
    }
}

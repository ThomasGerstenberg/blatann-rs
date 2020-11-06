#![allow(dead_code)]

#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate lazy_static;

pub mod nrf_error;
pub mod common;
pub mod gap;
pub mod ble_event;
pub mod event_publisher;
pub mod nrf_driver;
pub mod nrf_driver_manager;
mod utils;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod driver;  // Auto-genned C bindings

use std::sync::{Mutex};
use crate::nrf_driver_manager::NrfDriverManager;


lazy_static! {
    pub static ref DRIVER_MANAGER: Mutex<NrfDriverManager> = Mutex::new(NrfDriverManager::new());
}

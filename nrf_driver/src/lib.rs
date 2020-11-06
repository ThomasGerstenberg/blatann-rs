#![allow(dead_code)]

#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate lazy_static;

pub mod error;
pub mod common;
pub mod gap;
pub mod ble_event;
pub mod event_publisher;
pub mod driver;
pub mod manager;
pub mod utils;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod ffi;  // Auto-genned C bindings

use std::sync::{Mutex};
use crate::manager::NrfDriverManager;


lazy_static! {
    pub static ref DRIVER_MANAGER: Mutex<NrfDriverManager> = Mutex::new(NrfDriverManager::new());
}

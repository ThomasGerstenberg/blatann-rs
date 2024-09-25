extern crate bindgen;

use std::env;
use std::path::PathBuf;

const NRF_DRIVER_VERSION: &str = "4.1.2";
const SD_API_VERSION: usize = 5;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let version_underscore = NRF_DRIVER_VERSION.replace(".", "_");
    println!("cargo:rustc-link-search=native={}/external/nrf-ble-driver/nrf-ble-driver-{}-win_x86_64/lib", manifest_dir, NRF_DRIVER_VERSION);
    println!(
        "cargo:rustc-link-lib=nrf-ble-driver-sd_api_v{}-mt-static-{}",
        SD_API_VERSION, version_underscore
    );
    let bindings = bindgen::Builder::default()
        .header("external/wrapper.h")
        .generate_comments(false)
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(manifest_dir);
    bindings
        .write_to_file(out_path.join("src").join("ffi.rs"))
        .expect("Failed to write bindings");
}

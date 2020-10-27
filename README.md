# blatann-rs

Rust version of [blatann](https://github.com/ThomasGerstenberg/blatann)
and bindings for [pc-ble-driver](https://github.com/NordicSemiconductor/pc-ble-driver).

Created primarily as a method to learn rust but goal is to have pairity with the python version eventually.


Near-term goals:
- NrfDriver implementation, fully wrapping pc-ble-driver
- Event dispatching system
- BLE Advertising/scanning
- Multiplatform support (Only Windows+MSVC compiler is configured currently)

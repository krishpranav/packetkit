#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate packetkit;

fuzz_target!(|data: &[u8]| {
    let _ = packetkit::centrifuge::parse_eth(&data);
});

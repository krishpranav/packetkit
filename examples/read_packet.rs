extern crate base64;
extern crate packetkit;
extern crate env_logger;

use std::env;

fn main() {
    env_logger::init();

    for arg in env::args().skip(1) {
        let bytes = base64::decode(&arg).unwrap();
        println!("{:?}", bytes);

        let packet = packetkit::centrifuge::parse_eth(&bytes);
        println!("{:?}", packet);
    }
}

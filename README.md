# packetkit
An Advanced Network Packet Sniffer Built In Rust

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)

## Installation
```
$ git clone https://github.com/krishpranav/packetkit
$ cd packetkit
$ cargo build
```

## Usage:
```bash
# sniff with default filters (dhcp, dns, tls, http)
$ ./packetkit en0

# increase the filter sensitivity (arp)
$ packetkit -v en0

# increase the filter sensitivity (cjdns, ssdp, dropbox, packets with valid utf8)
$ packetkit -vv en0

# almost everything
$ packetkit -vvv en0

# everything
$ packetkit -vvvv en0
```

## About packetkit:

```
packetkit is a network sniffer written in rust. Network packets are parsed concurrently using a thread pool to utilize all cpu cores.
Project goals are that you can run packetkit securely on untrusted networks and that it must not crash when processing packets.
The output should be as useful as by default.
```

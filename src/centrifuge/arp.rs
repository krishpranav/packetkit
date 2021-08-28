use crate::structs::CentrifugeError;
use crate::structs::arp::ARP;

pub fn extract(remaining: &[u8]) -> Result<ARP, CentrifugeError> {
    if let Ok((_remaining, arp_pkt)) = pktparse::arp::parse_arp_pkt(remaining) {
        use pktparse::arp::Operation;
        match arp_pkt.operation {
            Operation::Request  => Ok(ARP::Request(arp_pkt)),
            Operation::Reply    => Ok(ARP::Reply(arp_pkt)),
            Operation::Other(_) => Err(CentrifugeError::UnknownProtocol), // TODO
        }
    } else {
        Err(CentrifugeError::InvalidPacket)
    }
}
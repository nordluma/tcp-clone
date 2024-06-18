use etherparse::{Ipv4HeaderSlice, TcpHeaderSlice};

#[derive(Debug, Default)]
pub struct State {}

impl State {
    pub fn on_packet<'a>(
        &mut self,
        ip_header: Ipv4HeaderSlice<'a>,
        tcp_header: TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) {
        eprintln!(
            "{}:{} -> {}:{} {}b of tcp to port",
            ip_header.source_addr(),
            tcp_header.source_port(),
            ip_header.destination_addr(),
            tcp_header.destination_port(),
            data.len(),
        );
    }
}

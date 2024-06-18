use std::{collections::HashMap, io, net::Ipv4Addr};

use tun_tap::Iface;

const HEADER_LEN: usize = 4;

mod tcp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Quad {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

fn main() -> io::Result<()> {
    let mut connections: HashMap<Quad, tcp::State> = Default::default();
    let nic = Iface::new("tun0", tun_tap::Mode::Tun)?;

    let mut buf = [0_u8; 1504];
    loop {
        let Ok(n_bytes) = nic.recv(&mut buf[..]) else {
            break;
        };

        let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        if eth_proto != 0x0800 {
            // no ipv4
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[HEADER_LEN..n_bytes]) {
            Ok(iph) => {
                let src = iph.source_addr();
                let dst = iph.destination_addr();
                if iph.protocol().0 != 0x06 {
                    // no tcp
                    continue;
                }

                match etherparse::TcpHeaderSlice::from_slice(&buf[HEADER_LEN + iph.slice().len()..])
                {
                    Ok(tcph) => {
                        let datai = HEADER_LEN + iph.slice().len() + tcph.slice().len();
                        connections
                            .entry(Quad {
                                src: (src, tcph.source_port()),
                                dst: (dst, tcph.destination_port()),
                            })
                            .or_default()
                            .on_packet(iph, tcph, &buf[..datai]);

                        // (srcip, srcport, dstip, dstport)
                    }
                    Err(e) => eprintln!("ignoring weird tcp packet {:?}", e),
                }
            }
            Err(e) => eprintln!("ignoring weird packet {:?}", e),
        }
    }

    Ok(())
}

use std::io;

use tun_tap::Iface;

const HEADER_LEN: usize = 4;

fn main() -> io::Result<()> {
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
            Ok(p) => {
                let src = p.source();
                let dst = p.destination_addr();
                let proto = p.protocol();
                if proto.0 != 0x06 {
                    // no tcp
                    continue;
                }

                match etherparse::TcpHeaderSlice::from_slice(&buf[HEADER_LEN + p.slice().len()..]) {
                    Ok(p) => {
                        eprintln!(
                            "{:?} -> {} {}b of tcp to port {}",
                            src,
                            dst,
                            p.slice().len(),
                            p.destination_port()
                        );
                    }
                    Err(e) => eprintln!("ignoring weird tcp packet {:?}", e),
                }
            }
            Err(e) => eprintln!("ignoring weird packet {:?}", e),
        }
    }

    Ok(())
}

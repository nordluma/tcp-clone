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

                eprintln!(
                    "{:?} -> {}b of protocol: {:?} {:?}",
                    src,
                    dst,
                    proto,
                    p.payload_len()
                );
            }
            Err(e) => eprintln!("ignoring weird packet {:?}", e),
        }
    }

    Ok(())
}

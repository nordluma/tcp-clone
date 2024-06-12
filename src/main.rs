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

        let flags = u16::from_be_bytes([buf[0], buf[1]]);
        let proto = u16::from_be_bytes([buf[2], buf[3]]);
        if proto != 0x0800 {
            // no ipv4
            continue;
        }

        eprintln!(
            "read {} bytes (flags: {:x}, proto: {:x}): {:x?}",
            n_bytes - HEADER_LEN,
            flags,
            proto,
            &buf[HEADER_LEN..n_bytes]
        );
    }

    Ok(())
}

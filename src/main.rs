use std::io;

use tun_tap::Iface;

fn main() -> io::Result<()> {
    let nic = Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0_u8; 1504];
    let n_bytes = nic.recv(&mut buf[..])?;
    eprintln!("read {} bytes: {:x?}", n_bytes, &buf[..n_bytes]);

    Ok(())
}

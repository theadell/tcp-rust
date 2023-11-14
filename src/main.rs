use std::io;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let flags = u16::from_be_bytes([buf[0], buf[1]]);
        let proto = u16::from_be_bytes([buf[2], buf[3]]);
        if proto != 0x800 {
            // Not IPV4 packet
            continue;
        }
        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(p) => {
                eprintln!(
                    "read {} bytes (flags: {:x}, proto: {:x}): {:?}",
                    nbytes - 4,
                    flags,
                    proto,
                    p
                );
            }
            Err(e) => {
                eprint!("Ignoring package {:?}", e);
            }
        }
    }
    // some comment added
    // Ok(())
}

use std::collections::HashMap;
use std::io;
use std::net::Ipv4Addr;

mod tcp;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Quad {
    src: (Ipv4Addr, u16),
    dstn: (Ipv4Addr, u16),
}

fn main() -> io::Result<()> {
    let mut connections: HashMap<Quad, tcp::State> = Default::default();
    let mut nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop {
        // link level stuff

        let nbytes = nic.recv(&mut buf[..])?;
        // read flags and proto of the ethernet frame
        let _flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        if eth_proto != 0x800 {
            // Not IPV4 packet
            continue;
        }
        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(iph) => {
                let source = iph.source_addr();
                let dstn = iph.destination_addr();
                if iph.protocol() != 0x06 {
                    continue;
                }

                // read the tcp header which comes after the ip header (4 bytes link header + the
                // size of the ipv4 packet header
                match etherparse::TcpHeaderSlice::from_slice(&buf[4 + iph.slice().len()..nbytes]) {
                    Ok(tcph) => {
                        let datai = 4 + iph.slice().len() + tcph.slice().len();
                        // get the connection state or create one if it is new
                        connections
                            .entry(Quad {
                                src: (source, tcph.source_port()),
                                dstn: (dstn, tcph.destination_port()),
                            })
                            .or_default()
                            .on_packet(&mut nic, iph, tcph, &buf[datai..nbytes])?;
                    }
                    Err(e) => {
                        eprint!("Ignoring tcp package {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprint!("Ignoring package {:?}", e);
            }
        }
    }
    // some comment added
    // Ok(())
}

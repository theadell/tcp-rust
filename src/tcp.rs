use std::io;

pub enum State {
    Closed,
    Listen,
    //SynRcvd,
    // Esab,
}

impl Default for State {
    fn default() -> Self {
        // State::Closed
        State::Listen
    }
}

impl State {
    pub fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<usize> {
        let mut buf = [0u8; 1500];
        match *self {
            State::Closed => {
                return Ok(0);
            }
            State::Listen => {
                if !tcph.syn() {
                    // only expected syn package
                    return Ok(0);
                }
                // now here we need to start establishing a connection
                let mut syn_ack =
                    etherparse::TcpHeader::new(tcph.destination_port(), tcph.source_port(), 0, 0);
                // SYN ACK
                syn_ack.syn = true;
                syn_ack.ack = true;

                // the ip packet enveloping the tcp message
                let mut ip = etherparse::Ipv4Header::new(
                    syn_ack.header_len(),
                    64,
                    etherparse::IpTrafficClass::Tcp,
                    [
                        iph.destination()[0],
                        iph.destination()[1],
                        iph.destination()[2],
                        iph.destination()[3],
                    ],
                    [
                        iph.source()[0],
                        iph.source()[1],
                        iph.source()[2],
                        iph.source()[3],
                    ],
                );

                // Calculate the length of the unwritten part of the buffer after writing headers.
                // The result of this code block is assigned to the variable `unwritten`.
                let unwritten = {
                    // Create a mutable slice of the entire buffer.
                    // This slice will be used to write data into the buffer and will be updated
                    // as data is written, pointing to the remaining "unwritten" part of the buffer.
                    let mut unwritten = &mut buf[..];

                    // Write the IP header into the buffer. This method modifies the `unwritten` slice
                    // to point to the part of the buffer that is still unwritten, i.e., after the IP header.
                    ip.write(&mut unwritten);

                    // Write the SYN-ACK data into the buffer, further modifying the `unwritten` slice
                    // to point to the part of the buffer after the SYN-ACK data.
                    syn_ack.write(&mut unwritten);

                    // After the headers are written, get the length of the remaining unwritten portion
                    // of the buffer. This length will be used to determine how much of the buffer to send.
                    unwritten.len()
                };
                // Send the written part of the buffer over the network interface (`nic`).
                // The slice `&buf[..unwritten]` includes everything from the start of `buf` to the
                // point marked by `unwritten`, which is the end of the written data (headers).
                nic.send(&buf[..unwritten])
            }
            // State::SynRcvd => todo!(),
            // State::Esab => todo!(),
        }
    }
}

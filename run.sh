#!/bin/bash
cargo b --release 
ext=$?
if [[ $ext -ne 0 ]]; then
	exit $ext
fi
setcap cap_net_admin=eip target/release/trust
./target/release/trust &
pid=$! 
ip addr add 192.168.0.1/24 dev tun0
ip link set up dev tun0
trap "kill $pid" INT TERM
wait $pid

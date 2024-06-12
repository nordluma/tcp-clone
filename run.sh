#!/bin/sh
cargo b --release
sudo setcap cap_net_admin=eip $HOME/dev/rust/tcp-clone/target/release/tcp-clone
$HOME/dev/rust/tcp-clone/target/release/tcp-clone &
pid=$!
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
wait $pid

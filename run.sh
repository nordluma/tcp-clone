#!/bin/bash
cargo b --release

ext=$?
echo "$ext"
if [[ $ext -ne 0 ]]; then 
    exit $ext
fi

sudo setcap cap_net_admin=eip $HOME/dev/rust/tcp-clone/target/release/tcp-clone
$HOME/dev/rust/tcp-clone/target/release/tcp-clone &
pid=$!
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
trap 'kill $pid' INT TERM
wait $pid

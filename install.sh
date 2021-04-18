#!/usr/bin/env bash
set -e

nice cargo build --release

# systemd stuff
sudo mkdir -p /usr/local/lib/systemd/system/
sudo cp -uv ./*.service /usr/local/lib/systemd/system/
sudo systemctl daemon-reload

# copy to system
sudo systemctl stop network-stalker.service
sudo cp -uv target/release/network-stalker /usr/local/bin/

sudo systemctl start network-stalker.service

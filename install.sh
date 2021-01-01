#!/bin/sh

nice cargo build --release

# systemd stuff
sudo cp -uv *.service /etc/systemd/system
sudo systemctl daemon-reload

# copy to system
sudo systemctl stop network-stalker.service
sudo cp -uv target/release/network-stalker /usr/local/bin/

sudo systemctl start network-stalker.service

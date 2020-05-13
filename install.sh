#!/bin/sh

nice cargo build --release
sudo cp -uv target/release/network-stalker /usr/local/bin/

# copy stuff
sudo cp -uv *.service /etc/systemd/system

# reload systemd
sudo systemctl daemon-reload

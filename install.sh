#!/bin/sh

sudo cargo install --root /usr/local/bin/

# copy stuff
sudo cp -uv *.service /etc/systemd/system

# reload systemd
sudo systemctl daemon-reload

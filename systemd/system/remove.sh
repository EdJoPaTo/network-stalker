#!/usr/bin/env bash

name="network-stalker"

sudo systemctl disable --now "$name.service"

sudo rm -f "/usr/local/lib/systemd/system/$name.service"
sudo rm -f "/usr/local/bin/$name"

sudo userdel -r "$name"
sudo groupdel "$name"

sudo systemctl daemon-reload


echo "/var/local/lib/network-stalker/ is not touched and is still existing"

#!/usr/bin/env bash

sudo systemctl disable --now "network-stalker.service"

sudo rm -f "/usr/local/lib/systemd/system/network-stalker.service"
sudo rm -f "/usr/local/bin/network-stalker"

sudo systemctl daemon-reload

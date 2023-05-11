#!/bin/bash

mkdir -p /var/log/cron-launcher
mkdir -p /opt/cron-launcher/bin/
cp -rv conf /opt/cron-launcher/
cp -v ./cron-launcher /opt/cron-launcher/bin/

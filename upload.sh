#!/bin/bash
cargo nx build
ncftpput -P 5000 -u nxuser -p nxsecret 192.168.5.26 /switch/ target/*/debug/nx-sys-mqtt.nro

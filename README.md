
# nx-sys-mqtt

This project is intended to be a background service on a homebrew-enabled Nintendo Switch, which will communicate with an MQTT broker when certain actions are done with the console. What's possible or not is currently unknown, but below is what's targeted. It's the ambition to do this in Rust.

## Todo
* suspend
* wakeup
* game started
  - game name
  - game id
* game ended
  - duration
* docked
* undocked

## Acknowledgements

aarch64-switch-rs, for examples, guide and most important - the nx library and cargo subcommand.
https://github.com/aarch64-switch-rs

rumqtt for MQTT library
https://github.com/bytebeamio/rumqtt
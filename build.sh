#! /bin/bash

# exit if command fails
set -e

cross build --target arm-unknown-linux-gnueabi

scp target/arm-unknown-linux-gnueabi/debug/system_controller drake@192.168.50.106:~
scp src/config.toml drake@192.168.50.106:~

#! /bin/bash

# exit if command fails
set -e

cross build --target arm-unknown-linux-gnueabi
#echo $TEST
scp target/arm-unknown-linux-gnueabi/debug/system_controller drake@192.168.50.106:~

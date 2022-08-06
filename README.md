# [usb-device](https://github.com/rust-embedded-community/usb-device) testing resources
Running the full test suite for [usb-device](https://github.com/rust-embedded-community/usb-device) requires two computers, connected via USB: a target USB device, and a host running the test suite.  This repository contains firmware source code for that target device, and instructions to run the tests using that firmware.

In general, since the object is to test usb-device, we can't assume that the USB is a viable channel for flashing the firmware.  These firmwares are intended to be loaded via SWD, so a debug probe supported by [cargo-embed](https://github.com/probe-rs/cargo-embed) will be required.

## Adafruit Metro M4 Express
# [usb-device](https://github.com/rust-embedded-community/usb-device) testing resources
Running the full test suite for [usb-device](https://github.com/rust-embedded-community/usb-device) requires two computers, connected via USB: a target USB device, and a host running the test suite.  This repository contains firmware source code for that target device, and instructions to run the tests using that firmware.

In general, since the object is to test usb-device, we can't assume that the USB is a viable channel for flashing the firmware.  These firmwares are intended to be loaded via SWD, so a debug probe supported by [cargo-embed](https://github.com/probe-rs/cargo-embed) will be required, rather than using a protocol like HF2 to flash over USB.

This repo has `usb-device` as a git submodule, so either clone using the `--recursive` flag initially, or run `git submodule update --init` if the `usb-device` directory is empty.

## Required Hardware

```mermaid
graph LR;
  Debugger[Debug probe]
  Host --- Debugger
  Host ---|USB| Target
  Debugger ---|SWD| Target
```

### Adafruit Metro M0 Express

Firmware is included in the `metro-m0-express` directory for running the tests against an [Adafruit](https://www.adafruit.com/) [Metro M0](https://www.adafruit.com/product/3505) - based on ATSAMD21G18 Cortex M0+.  Loading this firmware will require a SWD debug probe.

### Adafruit Metro M4 Express

Firmware is included in the `metro-m4-express` directory for running the tests against an [Adafruit](https://www.adafruit.com/) [Metro M4](https://www.adafruit.com/product/3382) - based on ATSAMD21G18 Cortex M4.  Loading this firmware will require a SWD debug probe.

## Running the tests

 1. Update `usb-device` submodule to the commit to test
 2. Connect target board and programmer
 3. Compile and flash firmware in to the target as described in the README for the chosen target
 4. In the usb-device directory, run `cargo test`

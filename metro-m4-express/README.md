# Adafruit Metro M4 usb-device test firmware

Usually, this will be installed over SWD, a [cargo-embed](https://github.com/probe-rs/cargo-embed) config file is included.

To avoid potential timing-related issues, it's recommended to run the tests with release firmware.

## Prerequisites

```
$ rustup target add thumbv7em-none-eabihf
$ cargo install cargo-embed
```

## Installing
With the Metro M4 and a debug probe (a J-Link was used for initial development) connected via SWD, `cargo embed --release`
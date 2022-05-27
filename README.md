## Undergraduate Teaching Platform: for the [TI TM4C Launchpad](http://www.ti.com/tool/EK-TM4C123GXL)! 👷

[![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fut-utp%2Ftm4c%2Fbadge&style=for-the-badge)](https://github.com/ut-utp/tm4c/actions) [![License: MPL-2.0](https://img.shields.io/github/license/ut-utp/tm4c?color=orange&style=for-the-badge)](https://opensource.org/licenses/MPL-2.0)
--
[![](https://tokei.rs/b1/github/ut-utp/m4c)](https://github.com/ut-utp/tm4c) [![codecov](https://codecov.io/gh/ut-utp/tm4c/branch/master/graph/badge.svg)](https://codecov.io/gh/ut-utp/tm4c)

Uses [thejpster](https://github.com/thejpster/)'s [tm4c-hal crates](https://github.com/thejpster/tm4c-hal) heavily.

🐝 🚧 This is very much not stable yet! 🚧 🐝

## Usage

To be used in conjuction with the [UTP TUI](github.com/ut-utp/tui.git).

#### First: Flash your TM4C

You can grab a TM4C image (a `.bin` file; TODO: issue #7) from the [releases page](https://github.com/ut-utp/tm4c/releases).

You'll need to grab `lm4flash` and potentially install a driver in order to flash your TM4C. [This page](https://github.com/ut-utp/.github/wiki/Dev-Environment-Setup#for-the-tm4c) has instructions on how to do so.

Once you've done this, to flash your board run `lm4flash <path to the .bin file>`.

On macOS and Linux:
  - `lm4flash -v utp-tm4c.bin`
On Windows:
  - `lm4flash.exe -v utp-tm4c.bin`

At this point, if flashing the board was successful, your on-board LED should be blinking (TODO: issue #6).

(TODO: ulimately we want to switch to probe-rs and have the TUI handle this, actually...)

#### Next: Launch the TUI

First [install](https://github.com/ut-utp/tui#usage) the UTP TUI if you haven't already.

Next, find your device's serial port:
  - Windows: open device manager, look for COM ports, find the one that says stellaris
  - macOS: look in `/dev/` for something that starts with `/dev/cu.usbmodem`
  - Linux: `dmesg | tail` after you plug in or look in `/dev/` (probably something like `/dev/ttyACM0` if you don't have the [udev rule](https://github.com/ut-utp/.github/wiki/Dev-Environment-Setup#for-the-tm4c); otherwise `/dev/tm4c`)

And finally, run the TUI with the `--device board=<serial port path>:1500000` flag.

For example:
  - Windows: `utp-tui.exe --device board=COM11:1500000`
  - macOS: `./utp-tui --device board=/dev/cu.usbmodemABCD1234:1500000`
  - Linux: `./utp-tui --device board=/dev/tm4c:1500000`

(TODO: ultimately we want to streamline this to just `utp-tui --device tm4c`, tui#6)


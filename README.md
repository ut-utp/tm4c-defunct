## Undergraduate Teaching Platform: for the [TI TM4C Launchpad](http://www.ti.com/tool/EK-TM4C123GXL)! 👷

[![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fut-utp%2Ftm4c%2Fbadge&style=for-the-badge)](https://github.com/ut-utp/tm4c/actions) [![License: MPL-2.0](https://img.shields.io/github/license/ut-utp/tm4c?color=orange&style=for-the-badge)](https://opensource.org/licenses/MPL-2.0)
--
[![](https://tokei.rs/b1/github/ut-utp/m4c)](https://github.com/ut-utp/tm4c) [![codecov](https://codecov.io/gh/ut-utp/tm4c/branch/master/graph/badge.svg)](https://codecov.io/gh/ut-utp/tm4c)

Uses [thepster](https://github.com/thejpster/)'s [tm4c-hal crates](https://github.com/thejpster/tm4c-hal) heavily.

🐝 🚧 This is very much not stable yet! 🚧 🐝

To flash:
```bash
openocd \
    -c "source [find board/ek-tm4c123gxl.cfg]" \
    -c "init" \
    -c "halt" \
    -c "reset init" \
    -c "sleep 100" \
    -c "flash probe 0" \
    -c "flash write_image erase target/thumbv7em-none-eabihf/release/utp-tm4c" \
    -c "sleep 100" \
    -c "verify_image target/thumbv7em-none-eabihf/release/utp-tm4c" \
    -c "halt" \
    -c "shutdown"
```

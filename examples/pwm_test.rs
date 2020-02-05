#![no_main]
#![no_std]

extern crate panic_halt;
extern crate tm4c123x_hal as hal;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use hal::prelude::*;

use lc3_traits::peripherals::pwm::{
    Pwm, PwmPin, PwmPinArr, PwmSetDutyError, PwmSetPeriodError, PwmState,
};

use hal_shims::peripherals::pwm;
use hal_shims::peripherals::pwm::required_components;

#[entry]
fn main() -> ! {
    let p = hal::Peripherals::take().unwrap();
    let mut sc = p.SYSCTL;
    let mut portb = p.GPIO_PORTB;
    let mut pwm0 = p.PWM0;
    let mut pwm1 = p.PWM1;
    let mut pwm_shim = pwm::PwmShim::new(required_components {
        sysctl: sc,
        portb: portb,
        pwm0: pwm0,
        pwm1: pwm1,
    });
    pwm_shim.set_state(
        PwmPin::P0,
        PwmState::Enabled(core::num::NonZeroU8::new(150).unwrap()),
    );

    loop {}
}

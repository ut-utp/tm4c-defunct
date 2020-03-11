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

use lc3_tm4c::peripherals_tm4c::pwm;
use lc3_tm4c::peripherals_tm4c::pwm::required_components;
use tm4c123x_hal::sysctl;
use lc3_tm4c::peripherals_tm4c::gpio;
use lc3_traits::peripherals::gpio::{
    Gpio, GpioMiscError, GpioPin, GpioPinArr, GpioReadError, GpioState, GpioWriteError,
};

#[entry]
fn main() -> ! {
    let p = hal::Peripherals::take().unwrap();
    let mut sc = p.SYSCTL;
    let mut portb = p.GPIO_PORTB;
    let mut portd = p.GPIO_PORTD;
    let mut porta = p.GPIO_PORTF;
    let mut porte = p.GPIO_PORTE;
    let mut pwm0 = p.PWM0;
    let mut pwm1 = p.PWM1;
    let sys = sc.constrain();
    let mut pwm_shim = pwm::PwmShim::new(required_components {
        //sysctl: sc,
        portb: portb,
        portd: portd,
        pwm0: pwm0,
        pwm1: pwm1,
    }, &sys.power_control);
    pwm_shim.set_state(
        PwmPin::P0,
        PwmState::Enabled(core::num::NonZeroU8::new(150).unwrap()),
    );
    pwm_shim.set_state(
        PwmPin::P1,
        PwmState::Enabled(core::num::NonZeroU8::new(150).unwrap()),
    );
    let mut pins = gpio::physical_pins::new(
        &sys.power_control,
        gpio::required_components {
            porta: porta,
            porte: porte,
        },
    );
   pins.set_state(GpioPin::G0, GpioState::Output);
   pins.set_state(GpioPin::G1, GpioState::Output);
   pins.set_state(GpioPin::G2, GpioState::Output);
   pins.set_state(GpioPin::G3, GpioState::Output);

   pins.set_pin(GpioPin::G0, true);
   pins.set_pin(GpioPin::G1, true);
   pins.set_pin(GpioPin::G2, true);
   pins.set_pin(GpioPin::G3, true);
       //pwm_shim.set_state(
        //PwmPin::P1,
      //  PwmState::Enabled(core::num::NonZeroU8::new(150).unwrap()),
    //);

    loop {
    pins.set_pin(GpioPin::G0, true);
    pins.set_pin(GpioPin::G1, true);
    pins.set_pin(GpioPin::G2, true);
    pins.set_pin(GpioPin::G3, true);

    for pat in 0..1000000{
        //unimplemented!();
    }
    // pins.set_pin(GpioPin::G0, false);
    // pins.set_pin(GpioPin::G1, false);
    // pins.set_pin(GpioPin::G2, false);
    // pins.set_pin(GpioPin::G3, false);


    }
}

pub mod adc;
pub mod gpio;
pub mod gpio_singleton;
pub mod pwm;
pub mod timers;
pub mod clock;
pub mod flash;
pub mod gpio_tm4c_impl;
pub mod adc_tm4c_hal_temp;

use lc3_traits::peripherals::stubs;
use lc3_traits::peripherals::stubs::*;
use lc3_traits::peripherals::PeripheralSet;

pub type Peripheralstm4c<'s> = PeripheralSet<
    's,
    GpioStub,
    AdcStub,
    pwm::PwmShim,
    TimersStub,
    ClockStub,
    InputStub,
    OutputStub,
>;
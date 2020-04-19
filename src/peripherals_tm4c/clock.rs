use core::convert::TryInto;
use lc3_isa::{Word, WORD_MAX_VAL};
use lc3_traits::peripherals::clock::Clock;

use tm4c123x::*;
use tm4c123x_hal::sysctl;
//use crate::timers::*;

 pub struct Tm4cClock;

 pub struct required_components{
    pub timer: tm4c123x::TIMER2,
 }

static mut CURRENT_TIME_MSECS: u16 = 0;

impl Default for Tm4cClock {
    fn default() -> Self {
             let t2 = unsafe { &*tm4c123x::TIMER2::ptr() };
             let p = unsafe { &*tm4c123x::SYSCTL::ptr() };
             p.rcgctimer.write(|w| unsafe{w.bits(p.rcgctimer.read().bits() | 4)});  //activate timer0, 1
             t2.ctl.write(|w| unsafe{w.bits(0)});
             t2.cfg.write(|w| unsafe{w.bits(0)});
             t2.tamr.write(|w| unsafe{w.bits(2)});
             t2.tailr.write(|w| unsafe{w.bits(80000)});  // 1 msec precision; Assumes 80Mhz bus speed
             t2.tapr.write(|w| unsafe{w.bits(0)});
             t2.icr.write(|w| unsafe{w.bits(1)});
             t2.imr.write(|w| unsafe{w.bits(1)});
             t2.ctl.write(|w| unsafe{w.bits(1)});
             Tm4cClock

    }
}

impl Clock for Tm4cClock {
    fn get_milliseconds(&self) -> Word {
        unsafe{
        CURRENT_TIME_MSECS
        }

    }

    // they set milliseconds - adding to the current time,
    // next time that they call get_milliseconds(),
    // they will get the input milliseconds
    fn set_milliseconds(&mut self, ms: Word) {
    unsafe{
        CURRENT_TIME_MSECS = ms; 
     }
    }
}

impl Tm4cClock {
    pub fn new(peripheral_set:required_components, power: &sysctl::PowerControl) -> Self{
        let t = peripheral_set.timer;

        Tm4cClock

    }
}


#[interrupt]
fn TIMER2A(){
    unsafe{
        CURRENT_TIME_MSECS += 1; 
    }

}
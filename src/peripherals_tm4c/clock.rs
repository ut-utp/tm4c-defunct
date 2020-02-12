use core::convert::TryInto;
use lc3_isa::{Word, WORD_MAX_VAL};
use lc3_traits::peripherals::clock::Clock;

use tm4c123x::*;
//use crate::timers::*;

 pub struct ClockShim{
     clock_freq: u32,
     //clock:      tm4c123x::TIMER2,
 }

impl Default for ClockShim {
    fn default() -> Self {
             let t2 = unsafe { &*tm4c123x::TIMER2::ptr() };
             let p = unsafe { &*tm4c123x::SYSCTL::ptr() };
             p.rcgctimer.write(|w| unsafe{w.bits(p.rcgctimer.read().bits() | 4)});  //activate timer0, 1
             t2.ctl.write(|w| unsafe{w.bits(0)});
             t2.cfg.write(|w| unsafe{w.bits(0)});
             t2.tamr.write(|w| unsafe{w.bits(2)});
             t2.tailr.write(|w| unsafe{w.bits(80000000)});  // 1 msec precision
             t2.tapr.write(|w| unsafe{w.bits(0)});
             t2.icr.write(|w| unsafe{w.bits(1)});
             t2.imr.write(|w| unsafe{w.bits(1)});
             t2.ctl.write(|w| unsafe{w.bits(1)});
        Self {
            clock_freq: 1000,  //1000 Hertz TODO: Use the tm4c hal time unit structs for this
            //clock:      t2,

        }
    }
}

impl Clock for ClockShim {
    fn get_milliseconds(&self) -> Word {
        let t2 = unsafe { &*tm4c123x::TIMER2::ptr() };
        ((80000000 - (t2.tav.read().bits()))) as u16/(80000) as u16

    }

    // they set milliseconds - adding to the current time,
    // next time that they call get_milliseconds(),
    // they will get the input milliseconds
    fn set_milliseconds(&mut self, ms: Word) {
        // let time = Duration::from_millis(ms as u64);
        // self.start_time = Instant::now().checked_sub(time).unwrap();
    }
}
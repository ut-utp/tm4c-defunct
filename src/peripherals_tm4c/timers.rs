use lc3_traits::peripherals::timers::{
    TimerArr, TimerId, TimerMiscError, TimerState, TimerStateMismatch, Timers,
};

// timing errors occuring during scan cycles (input and ouput errors)
// errors handling overwriting handlers? Can timers have multiple handlers?
use lc3_isa::Word;
//use std::time::Duration;
use core::sync::atomic::AtomicBool;
//use std::sync::atomic::Ordering;
use tm4c123x_hal::{timer, timer::*, timer::Timer, time::*};
use tm4c123x_hal::tm4c123x::{TIMER0, TIMER1};
use core::marker::PhantomData;

use tm4c123x_hal::{Peripherals, prelude::*};
use tm4c123x_hal::time::MegaHertz;
use tm4c123x_hal::sysctl::Clocks;
use tm4c123x::NVIC as nvic;
//use time;
//use timer;
 enum PhysicalTimers{
    T0(Timer<TIMER0>),
    T1(Timer<TIMER1>)
 }

 pub struct required_components{
    timer0: tm4c123x::TIMER0,
    timer1: tm4c123x::TIMER1,
 }

 // pub system_clock: tm4c123x_hal::sysctl::Sysctl = {
   
 //    tm4c123x_hal::Peripherals::take().unwrap().SYSCTL.constrain()
 // };
// The term “Single Shot” signifies a single pulse output of some duration.
 pub struct TimersShim<'a> {
     states: TimerArr<TimerState>,
     times: TimerArr<Word>,
     //phys_timers: Vec<PhysicalTimers>,
     flags: TimerArr<Option<&'a AtomicBool>>,
     //system_clock: tm4c123x_hal::sysctl::Sysctl,
     clock_freq: u32,
     //power: tm4c123x_hal::sysctl::PowerControl
     //sys_config: (u32, tm4c123x_hal::sysctl::PowerControl)
     //timer_struct: [Timer; 2]

     //guards: TimerArr<Option<timer::Guard>>,
 }

 // fn initialize_system_clock() -> &'static tm4c123x_hal::sysctl::Sysctl{
 //    let p = tm4c123x_hal::Peripherals::take().unwrap();

 //    let mut sc = p.SYSCTL.constrain();
 //    sc.clock_setup.oscillator = tm4c123x_hal::sysctl::Oscillator::Main(
 //        tm4c123x_hal::sysctl::CrystalFrequency::_16mhz,
 //        tm4c123x_hal::sysctl::SystemClock::UsePll(tm4c123x_hal::sysctl::PllOutputFrequency::_20mhz),
 //    );
 //    let clocks = sc.clock_setup.freeze();
 //    &sc
 // }

 impl Default for TimersShim<'_> {

    fn default()->Self{
            let t1 = Peripherals::take().unwrap().TIMER0;
        let mut sc = Peripherals::take().unwrap().SYSCTL.constrain();
    sc.clock_setup.oscillator = tm4c123x_hal::sysctl::Oscillator::Main(
        tm4c123x_hal::sysctl::CrystalFrequency::_16mhz,
        tm4c123x_hal::sysctl::SystemClock::UsePll(tm4c123x_hal::sysctl::PllOutputFrequency::_20mhz),
    );
    let clock = sc.clock_setup.freeze();  // changeable or are the frequencies fixed?
    let hz = clock.sysclk;
    let tm4c123x_hal::time::Hertz(freq) = hz;
    let time_init1 = tm4c123x_hal::timer::Timer::timer0::<MegaHertz>(
        t1,
        MegaHertz(80),
        &sc.power_control,
        &clock,
    );
    let time_init2 = tm4c123x_hal::timer::Timer::timer1::<MegaHertz>(
        Peripherals::take().unwrap().TIMER1,
        MegaHertz(80),
        &sc.power_control,
        &clock,
    );
         Self {
             states: TimerArr([TimerState::Disabled; TimerId::NUM_TIMERS]),
             times: TimerArr([0u16; TimerId::NUM_TIMERS]), // unlike gpio, interrupts occur on time - not on bit change
             flags: TimerArr([None; TimerId::NUM_TIMERS]),
//             guards: TimerArr([None, None]),

             //phys_timers: vec![PhysicalTimers::T0(time_init1), PhysicalTimers::T1(time_init2)],
             clock_freq: freq,
             //power: sc.power_control
         }                

    }

 }

 impl TimersShim<'_> {

     pub fn new(power: &tm4c123x_hal::sysctl::PowerControl, peripheral_set: required_components, nvic_field: tm4c123x::NVIC) -> Self {

             let t1 = peripheral_set.timer0;
             let t2 = peripheral_set.timer1;
             let p = unsafe { &*tm4c123x::SYSCTL::ptr() };
             p.rcgctimer.write(|w| unsafe{w.bits(p.rcgctimer.read().bits() | 3)});  //activate timer0, 1
    //     let mut sc = Peripherals::take().unwrap().SYSCTL.constrain();
    // sc.clock_setup.oscillator = tm4c123x_hal::sysctl::Oscillator::Main(
    //     tm4c123x_hal::sysctl::CrystalFrequency::_16mhz,
    //     tm4c123x_hal::sysctl::SystemClock::UsePll(tm4c123x_hal::sysctl::PllOutputFrequency::_20mhz),
    // );
    //let clock = sc.clock_setup.freeze();  // changeable or are the frequencies fixed?
    // let hz = (*frozen_clock).sysclk;
    // let tm4c123x_hal::time::Hertz(freq) = hz;
    // let time_init1 = tm4c123x_hal::timer::Timer::timer0::<MegaHertz>(
    //     t1,
    //     MegaHertz(80),
    //     &((*system_config).power_control),
    //     frozen_clock,
    // );
    // let time_init2 = tm4c123x_hal::timer::Timer::timer1::<MegaHertz>(
    //     t2,
    //     MegaHertz(80),
    //     &((*system_config).power_control),
    //     frozen_clock,
    // );

        t1.ctl.write(|w| unsafe{w.bits(0)});
        t1.cfg.write(|w| unsafe{w.bits(0)});
        t1.tamr.write(|w| unsafe{w.bits(2)});
        t1.tailr.write(|w| unsafe{w.bits(80000)});
        t1.tapr.write(|w| unsafe{w.bits(0)});
        t1.icr.write(|w| unsafe{w.bits(1)});
        t1.imr.write(|w| unsafe{w.bits(1)});
        t1.ctl.write(|w| unsafe{w.bits(1)});

        //let nvicp = unsafe { &*tm4c123x::NVIC::ptr() };
        // let nvic_peripheral = tm4c123x::NVIC{
        //     _marker: core::marker::PhantomData,
        // };
        unsafe{nvic::unmask(tm4c123x::Interrupt::TIMER0A);};
       // nvic_field::NVIC.set_priority(tm4c123x::Interrupt::TIMER0A, 4);
       // unsafe{nvic::set_priority(tm4c123x::Interrupt::TIMER0A, 4);};

        t2.ctl.write(|w| unsafe{w.bits(0)});
        t2.cfg.write(|w| unsafe{w.bits(0)});
        t2.tamr.write(|w| unsafe{w.bits(2)});
        t2.tailr.write(|w| unsafe{w.bits(80000)});
        t2.tapr.write(|w| unsafe{w.bits(0)});
        t2.icr.write(|w| unsafe{w.bits(1)});
        t2.imr.write(|w| unsafe{w.bits(1)});
        t2.ctl.write(|w| unsafe{w.bits(1)});
       // let peri = tm4c123x::CorePeripherals::take().unwrap();
       // let mut nvic_field = peri.NVIC;
        let mut nvic_f = nvic_field;
        unsafe{nvic::unmask(tm4c123x::Interrupt::TIMER1A);};
        unsafe{nvic_f.set_priority(tm4c123x::Interrupt::TIMER1A, 4);};
         Self {
             states: TimerArr([TimerState::Disabled; TimerId::NUM_TIMERS]),
             times: TimerArr([0u16; TimerId::NUM_TIMERS]), // unlike gpio, interrupts occur on time - not on bit change
             flags: TimerArr([None; TimerId::NUM_TIMERS]),
//             guards: TimerArr([None, None]),

           //  phys_timers: vec![PhysicalTimers::T0(time_init1), PhysicalTimers::T1(time_init2)],
             clock_freq: 80_000_000,
             //power: sc.power_control
         }
     }

     
 }

 impl<'a> Timers<'a> for TimersShim<'a> {
     fn set_state(&mut self, timer: TimerId, state: TimerState) -> Result<(), TimerMiscError> {
        use TimerState::*;

        match state{
            Disabled => {
                match timer{
                    T0 =>{
                    let t0 = unsafe { &*tm4c123x::TIMER0::ptr() };  
                    t0.ctl.modify(|_, w|
                    w.taen().clear_bit()
                    .tben().clear_bit()
                    );                   
                    }

                    T1 => {
                    let t1 = unsafe { &*tm4c123x::TIMER1::ptr() }; 
                    t1.ctl.modify(|_, w|
                    w.taen().clear_bit()
                    .tben().clear_bit()
                    );
                    }
                }

            }

            _ => {}   // it's periodic by default
        }
        self.states[timer] = state;

         Ok(())
     }

    fn get_state(&self, timer: TimerId) -> TimerState {
        self.states[timer]
    }

     fn set_period(&mut self, timer: TimerId, milliseconds: Word) -> Result<(), TimerMiscError> {




//         //  use TimerState::*;

          self.times[timer] = milliseconds;
           match timer{
             T0 => {
               // let curr_timer = self.phys_timers.remove(0);
               // match curr_timer{

                //PhysicalTimers::T0(mut v) =>{
                  // let clk_freq = self.clock_setup[0];
                  // let tp_millis = (1/clk_freq)*1000;
                  // let divider = (milliseconds as u32)/tp_millis;
                  // let ticks_new = clk_freq/divider;


                  // Disable before making changes
                  let t0 = unsafe { &*tm4c123x::TIMER0::ptr() };  
                    t0.ctl.modify(|_, w|
                    w.taen().clear_bit()
                    .tben().clear_bit()
                    );
                   t0.tav.write(|w| unsafe { w.bits(millis_to_ticks(milliseconds as f32, self.clock_freq as f32)) });
                   t0.tailr.write(|w| unsafe { w.bits(millis_to_ticks(milliseconds as f32, self.clock_freq as f32)) });

                    // // start counter
                    t0.ctl.modify(|_, w|
                        w.taen().set_bit()
                    );
                    //self.phys_timers.insert(0,PhysicalTimers::T0(v));
               // }
                //_=> {}
            //}
             }

             T1 => {
               // let curr_timer = self.phys_timers.remove(0);
               // match curr_timer{
               // PhysicalTimers::T1(mut v) =>{

                    let t1 = unsafe { &*tm4c123x::TIMER1::ptr() };  
                    t1.ctl.modify(|_, w|
                    w.taen().clear_bit()
                    .tben().clear_bit()
                    );
                   t1.tav.write(|w| unsafe {  w.bits(millis_to_ticks(milliseconds as f32, self.clock_freq as f32)) });
                   Peripherals::take().unwrap().TIMER1.tailr.write(|w| unsafe { w.bits(millis_to_ticks(milliseconds as f32, self.clock_freq as f32)) });

                    // // start counter
                    t1.ctl.modify(|_, w|
                        w.taen().set_bit()
                    );
                    //self.phys_timers.insert(1,PhysicalTimers::T1(v));
                //}
                //_=> {}
            //}
      
             }

           }
       //   let sys_sp = self.phys_timers[T0].clocks;
//         //  let timer_init = timer::Timer::new();

         // match self.guards[timer] {
//         //      Some(_) => {
//         //          let g = self.guards[timer].take().unwrap();
//         //          drop(g);

//         //      },
//         //      None => {}
//         //  }

//         //  match self.states[timer] {
//         //      Repeated => {
//         //          match self.flags[timer] {
//         //              Some(b) => {
//         //                  let guard = {
//         //                      timer_init.schedule_repeating(time::Duration::milliseconds(milliseconds as i64), move || {
//         //                      //self.flags[timer].unwrap().store(true, Ordering::SeqCst);
//         //                      b.store(true, Ordering::SeqCst);
//         //                      })

//         //                  };

//         //                  self.guards[timer] = Some(guard);
//         //              },
//         //              None => {
//         //                  unreachable!();
//         //              }

//         //          }
//         //      },
//         //      SingleShot => {
//         //          match self.flags[timer] {
//         //              Some(b) => {
//         //                  let guard = {
//         //                          timer_init.schedule_with_delay(time::Duration::milliseconds(milliseconds as i64), move || {
//         //                      //self.flags[timer].unwrap().store(true, Ordering::SeqCst);
//         //                          b.store(true, Ordering::SeqCst);
//         //                      })
//         //                   };

//         //                   self.guards[timer] = Some(guard);
//         //              }
//         //              None => {
//         //                  unreachable!();
//         //              }
//         //          }
//         //      },
           //    Disabled => {
            //       unreachable!();
            //   }

//         //  }

           Ok(())

//         unimplemented!()
     }

    fn get_period(&self, timer: TimerId) -> Word {
        self.times[timer]
    }

    fn register_interrupt_flags(&mut self, flags: &'a TimerArr<AtomicBool>){
        // self.flags[timer] = match self.flags[timer] {
        //     None => Some(flag),
        //     Some(_) => unreachable!(),
        // }
    }

    fn interrupt_occurred(&self, timer: TimerId) -> bool {
        // match self.flags[timer] {
        //     Some(flag) => {
        //         let occurred = flag.load(Ordering::SeqCst);
        //         self.interrupts_enabled(timer) && occurred
        //     }
        //     None => unreachable!(),
        // }
        false
    }

    fn reset_interrupt_flag(&mut self, timer: TimerId) {
        // match self.flags[timer] {
        //     Some(flag) => flag.store(false, Ordering::SeqCst),
        //     None => unreachable!(),
        // }
    }

    // TODO: review whether we want Interrupt state or interrupts_enabled bool state
    fn interrupts_enabled(&self, timer: TimerId) -> bool {
        match self.get_state(timer) {
            SingleShot => true,
            Repeating => true,
            Disabled => false,
        }
    }
 }


 fn ticks_to_millis (ticks: f32, freq: f32)->u32{
    ((1.0/freq)*ticks*1000.0) as u32    // enable fpu on board for this part; better accuracy

 }

 fn millis_to_ticks (millis: f32, freq: f32)->u32{
    //TODO check for valid parameter ranges ^
    ((millis/1000.0)/(1.0/freq)) as u32  //assumes 1 tick per clock cycle which is default. I see no reason to use different divider

 }

// // #[cfg(test)]
// // mod tests {
// //     use super::*;
// //     use lc3_traits::peripherals::timers::{Timer::*, Timers};

// //     #[test]
// //     fn get_disabled() {
// //         let shim = TimersShim::new();
// //         assert_eq!(shim.get_state(T0).unwrap(), TimerState::Disabled);
// //     }

// //     #[test]
// //      fn get_singleshot() {
// //         let mut shim = TimersShim::new();
// //         let res = shim.set_state(T0, TimerState::SingleShot);
// //         assert_eq!(shim.get_state(T0).unwrap(), TimerState::SingleShot);
// //     }

// //     #[test]
// //      fn get_repeated() {
// //         let mut shim = TimersShim::new();
// //         let res = shim.set_state(T0, TimerState::Repeated);
// //         assert_eq!(shim.get_state(T0).unwrap(), TimerState::Repeated);
// //     }

// //     #[test]
// //      fn get_set_period_singleshot() {
// //         let mut shim = TimersShim::new();
// //         let res = shim.set_state(T0, TimerState::SingleShot);
// //         shim.set_period(T0, 200);
// //         assert_eq!(shim.get_period(T0).unwrap(), 200);
// //     }

// //     #[test]
// //      fn get_set_period_repeated() {
// //         let mut shim = TimersShim::new();
// //         let res = shim.set_state(T0, TimerState::Repeated);
// //         shim.set_period(T0, 200);
// //         assert_eq!(shim.get_period(T0).unwrap(), 200);
// //     }

// // }

fn sys_init() -> tm4c123x_hal::sysctl::PowerControl{
    let p_st = Peripherals::take().unwrap();
    let mut sc = p_st.SYSCTL.constrain();
    sc.power_control
   // tm4c123x_hal::sysctl::PowerControl{_0: ()}
}

fn scratch(){

    let t = Peripherals::take().unwrap().TIMER0;
    t.ctl.modify(|_, w|
                        w.taen().clear_bit()
                        .tben().clear_bit()
                        );

    let time_init = tm4c123x_hal::timer::Timer::timer0::<MegaHertz>(
        t,
        MegaHertz(80),
        &sys_init(),
        &Clocks{osc:Hertz(80000000), sysclk:Hertz(80000000)},
    );
}


use cortex_m_rt_macros::interrupt;
use tm4c123x::Interrupt as interrupt;

#[interrupt]
fn TIMER0A(){
    static mut COUNT: u32 = 0;

    // `COUNT` has type `&mut u32` and it's safe to use
    *COUNT += 1;

}

#[interrupt]
fn TIMER1A(){

}

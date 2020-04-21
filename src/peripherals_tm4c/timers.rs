use lc3_traits::peripherals::timers::{
    TimerArr, TimerId, TimerState, Timers, TimerMode
};
use lc3_isa::Word;
use core::sync::atomic::AtomicBool;
use tm4c123x_hal::{timer, timer::*, timer::Timer, time::*};
use tm4c123x_hal::tm4c123x::{TIMER0, TIMER1};
use core::marker::PhantomData;

use tm4c123x_hal::{Peripherals, prelude::*};
use tm4c123x_hal::time::MegaHertz;
use tm4c123x_hal::sysctl::Clocks;
use tm4c123x::NVIC as nvic;
extern crate cortex_m;
use cortex_m::interrupt as cortex_int;

static mut COUNT: u32 = 0;
static mut TIMER_INTERRUPTS: [u8; 2] = [0; 2];
 enum PhysicalTimers{
    T0(Timer<TIMER0>),
    T1(Timer<TIMER1>)
 }

 pub struct required_components{
    pub timer0: tm4c123x::TIMER0,
    pub timer1: tm4c123x::TIMER1,
 }

 pub struct TimersShim<'a> {
     states: TimerArr<TimerState>,
     times: TimerArr<Word>,
     modes: TimerArr<TimerMode>,
     flags: TimerArr<Option<&'a AtomicBool>>,
     clock_freq: u32,
 }


 impl Default for TimersShim<'_> {

    fn default()->Self{
            let t1 = Peripherals::take().unwrap().TIMER0;
        let mut sc = Peripherals::take().unwrap().SYSCTL.constrain();
    sc.clock_setup.oscillator = tm4c123x_hal::sysctl::Oscillator::Main(
        tm4c123x_hal::sysctl::CrystalFrequency::_16mhz,
        tm4c123x_hal::sysctl::SystemClock::UsePll(tm4c123x_hal::sysctl::PllOutputFrequency::_20mhz),
    );
    let clock = sc.clock_setup.freeze();  
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
             modes: TimerArr([TimerMode::SingleShot, TimerMode::SingleShot]),
             times: TimerArr([0u16; TimerId::NUM_TIMERS]), // unlike gpio, interrupts occur on time - not on bit change
             flags: TimerArr([None; TimerId::NUM_TIMERS]),
             clock_freq: freq,
         }                

    }

 }

 impl TimersShim<'_> {

     pub fn new(power: &tm4c123x_hal::sysctl::PowerControl, peripheral_set: required_components) -> Self {

        let t1 = peripheral_set.timer0;
        let t2 = peripheral_set.timer1;
        let p = unsafe { &*tm4c123x::SYSCTL::ptr() };
        p.rcgctimer.write(|w| unsafe{w.bits(p.rcgctimer.read().bits() | 3)});  //activate timer0, 1


        t1.ctl.write(|w| unsafe{w.bits(0)});
        t1.cfg.write(|w| unsafe{w.bits(0)});
        t1.tamr.write(|w| unsafe{w.bits(2)});
        t1.tailr.write(|w| unsafe{w.bits(80000)});
        t1.tapr.write(|w| unsafe{w.bits(0)});
        t1.icr.write(|w| unsafe{w.bits(1)});
        t1.imr.write(|w| unsafe{w.bits(1)});
       // t1.ctl.write(|w| unsafe{w.bits(1)});
        let mut nvic_field;
        unsafe{
        let p_core = tm4c123x_hal::CorePeripherals::steal();
        nvic_field = p_core.NVIC;
        };
       unsafe{nvic::unmask(tm4c123x::Interrupt::TIMER0A);};
       unsafe{nvic_field.set_priority(tm4c123x::Interrupt::TIMER0A, 1);};
       unsafe{nvic_field.enable(tm4c123x::Interrupt::TIMER0A);};


        t2.ctl.write(|w| unsafe{w.bits(0)});
        t2.cfg.write(|w| unsafe{w.bits(0)});
        t2.tamr.write(|w| unsafe{w.bits(2)});
        t2.tailr.write(|w| unsafe{w.bits(80000)});
        t2.tapr.write(|w| unsafe{w.bits(0)});
        t2.icr.write(|w| unsafe{w.bits(1)});
        t2.imr.write(|w| unsafe{w.bits(1)});
      //  t2.ctl.write(|w| unsafe{w.bits(1)});

        let mut nvic_f = nvic_field;
        unsafe{cortex_int::enable();};

         Self {
             states: TimerArr([TimerState::Disabled; TimerId::NUM_TIMERS]),
             modes: TimerArr([TimerMode::SingleShot, TimerMode::SingleShot]),
             times: TimerArr([0u16; TimerId::NUM_TIMERS]), 
             flags: TimerArr([None; TimerId::NUM_TIMERS]),
             clock_freq: 80_000_000,
         }
     }

     
 }

 impl<'a> Timers<'a> for TimersShim<'a> {

    fn set_mode(&mut self, timer: TimerId, mode: TimerMode){
        match mode{
            TimerMode::SingleShot => {
                match timer{
                    T0 =>{
                    let t0 = unsafe { &*tm4c123x::TIMER0::ptr() };  
                    t0.tamr.write(|w| unsafe{w.bits(1)});                
                    }

                    T1 => {
                    let t1 = unsafe { &*tm4c123x::TIMER1::ptr() }; 
                    t1.tamr.write(|w| unsafe{w.bits(1)});
                    }
                }

            }  

            TimerMode::Repeated => {
                match timer{
                    T0 =>{
                    let t0 = unsafe { &*tm4c123x::TIMER0::ptr() };  
                    t0.tamr.write(|w| unsafe{w.bits(2)});              
                    }

                    T1 => {
                    let t1 = unsafe { &*tm4c123x::TIMER1::ptr() }; 
                    t1.tamr.write(|w| unsafe{w.bits(2)});
                    }
                }
            }
        }     

    }
    fn get_mode(&self, timer: TimerId) -> TimerMode{
        self.modes[timer]
    }



     fn set_state(&mut self, timer: TimerId, state: TimerState){
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

             WithPeriod(mut period) =>{

                   match timer{
                     T0 => {
                        
                          let t0 = unsafe { &*tm4c123x::TIMER0::ptr() };  
                            t0.ctl.modify(|_, w|
                            w.taen().clear_bit()
                            .tben().clear_bit());

                           t0.tav.write(|w| unsafe { w.bits(period.get().into()) });
                           let per: u32 = period.get().into();

                           t0.tailr.write(|w| unsafe { w.bits(per*80000) });  //Assumes bus clock frequency is 80MHz

                           t0.ctl.modify(|_, w|
                                w.taen().set_bit()
                            );
                     }

                     T1 => {

                            let t1 = unsafe { &*tm4c123x::TIMER1::ptr() };  
                            t1.ctl.modify(|_, w|
                            w.taen().clear_bit()
                            .tben().clear_bit());

                           t1.tav.write(|w| unsafe {  w.bits(period.get().into()) });
                           let per: u32 = period.get().into();

                           Peripherals::take().unwrap().TIMER1.tailr.write(|w| unsafe { w.bits(per*80000) });
                            t1.ctl.modify(|_, w|
                                w.taen().set_bit());
              
                     }

                   }



             }   
        }
        self.states[timer] = state;
     }

    fn get_state(&self, timer: TimerId) -> TimerState {
        self.states[timer]
    }


    fn register_interrupt_flags(&mut self, flags: &'a TimerArr<AtomicBool>){

         unsafe{TIMER_INTERRUPTS = [0; 2];};
    }

    fn interrupt_occurred(&self, timer: TimerId) -> bool {

        let mut res = false;
        unsafe{
        if(TIMER_INTERRUPTS[usize::from(timer)]==1){
            res = true
        }
        else{
        res=false;
        }
    };
    res
    }

    fn reset_interrupt_flag(&mut self, timer: TimerId) {

         unsafe{TIMER_INTERRUPTS[usize::from(timer)]==0;};
    }

    fn interrupts_enabled(&self, timer: TimerId) -> bool {
        match self.get_state(timer) {
            SingleShot => true,
            Repeating => true,
            Disabled => false,
        }
    }
 }


 fn ticks_to_millis (ticks: f32, freq: f32)->u32{
    ((1.0/freq)*ticks*1000.0) as u32   

 }

 fn millis_to_ticks (millis: f32, freq: f32)->u32{
    ((millis/1000.0)/(1.0/freq)) as u32  
 }


fn sys_init() -> tm4c123x_hal::sysctl::PowerControl{
    let p_st = Peripherals::take().unwrap();
    let mut sc = p_st.SYSCTL.constrain();
    sc.power_control

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

    unsafe{

    let mut sc = &*tm4c123x::TIMER0::ptr();
    sc.icr.write(|w| unsafe{w.bits(1)});
    TIMER_INTERRUPTS[0]=1;


    //DEBUG

    let mut sc = &*tm4c123x::GPIO_PORTF::ptr();
    let bits = sc.ris.read().bits();
    let mut p = unsafe { &*tm4c123x::GPIO_PORTF::ptr() };
    let mut bits = p.data.read().bits();
    bits ^= 0x02;
    p.data.write(|w| unsafe { w.bits(bits) });  
};
    unsafe{COUNT += 1};

}



#[interrupt]
fn TIMER1A(){
    unsafe{
     let mut sc = &*tm4c123x::TIMER1::ptr();
    sc.icr.write(|w| unsafe{w.bits(1)}); 
    TIMER_INTERRUPTS[1]=1;      
    }

}

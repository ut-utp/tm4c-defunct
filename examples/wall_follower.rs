#![no_main]
#![no_std]

extern crate panic_halt;
extern crate tm4c123x_hal as hal;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use hal::prelude::*;
use core::fmt::Write;

use lc3_tm4c::persistent_data_management::page::Paging;
use lc3_tm4c::paging_impl::tm4c_flash_paging_config::*;
use lc3_tm4c::paging_impl::tm4c_flash_paging_config;
use lc3_tm4c::peripherals_tm4c::flash;
use lc3_tm4c::peripherals_tm4c::flash::*;

use lc3_tm4c::memory_impl::tm4c_memory_impl::*;


use lc3_baseline_sim::interp::*;

use lc3_traits::memory;
use lc3_traits::memory::*;
use lc3_traits::peripherals::stubs;
use lc3_traits::peripherals::stubs::*;
use lc3_traits::peripherals::PeripheralSet;

use lc3_traits::peripherals::pwm::{
    Pwm, PwmPin, PwmPinArr,  PwmState,
};


use lc3_traits::peripherals::adc::{
    Adc, AdcMiscError, AdcPin as Pin, AdcPinArr as PinArr, AdcReadError as ReadError, AdcState,
    AdcStateMismatch as StateMismatch,
};

use hal::adc as ad ;
use hal::{gpio::*, gpio::gpioe::*};
use lc3_tm4c::peripherals_tm4c::adc::required_components as adc_req;
use lc3_tm4c::peripherals_tm4c::adc as adc;

use lc3_tm4c::peripherals_tm4c::gpio;
use lc3_tm4c::peripherals_tm4c::gpio::required_components as gpio_req;
use lc3_traits::peripherals::gpio::{
    Gpio, GpioMiscError, GpioPin, GpioPinArr, GpioReadError, GpioState, GpioWriteError,
};
use lc3_tm4c::peripherals_tm4c::pwm;
use lc3_tm4c::peripherals_tm4c::pwm::required_components as pwm_req;

use lc3_tm4c::peripherals_tm4c::Peripheralstm4c;


use lc3_tm4c::peripherals_tm4c::timers;
use lc3_tm4c::peripherals_tm4c::timers::required_components as timer_req;

use lc3_tm4c::peripherals_tm4c::clock;
use lc3_tm4c::peripherals_tm4c::clock::required_components as clock_req;

use lc3_isa::{
    Addr, Instruction,
    Reg::{self, *},
    Word, ACCESS_CONTROL_VIOLATION_EXCEPTION_VECTOR, ILLEGAL_OPCODE_EXCEPTION_VECTOR,
    INTERRUPT_VECTOR_TABLE_START_ADDR, MEM_MAPPED_START_ADDR,
    PRIVILEGE_MODE_VIOLATION_EXCEPTION_VECTOR, TRAP_VECTOR_TABLE_START_ADDR,
    USER_PROGRAM_START_ADDR,
};

#[entry]
fn main() -> ! {
    let p = hal::Peripherals::take().unwrap();
 	    let p_core = hal::CorePeripherals::take().unwrap();
 	    let nvic = p_core.NVIC;
    let mut sc = p.SYSCTL;
    let mut sys = sc.constrain();
     sys.clock_setup.oscillator = hal::sysctl::Oscillator::Main(
        hal::sysctl::CrystalFrequency::_16mhz,
        hal::sysctl::SystemClock::UsePll(hal::sysctl::PllOutputFrequency::_80_00mhz),
    );
    let clocks = sys.clock_setup.freeze();
    let mut adc0 = p.ADC0;
    let mut adc1= p.ADC1;
    let mut pwm0 = p.PWM0;
    let mut pwm1 = p.PWM1;
    let mut flash = p.FLASH_CTRL;

    let mut portb = p.GPIO_PORTB;
    let mut portd = p.GPIO_PORTD;
    let mut portf = p.GPIO_PORTF;
    let mut porte = p.GPIO_PORTE;
  	let mut t0 = p.TIMER0;
 	let mut t1= p.TIMER1;
    let mut t2= p.TIMER2;
    
	let mut flash_unit = flash::tm4c_flash_unit{
		flash_ctrl: flash,
	};
    let mut adc_shim = adc::AdcShim::new(&sys.power_control, adc_req{adc0: adc0, adc1:adc1, porte: porte });
    let mut swap_obj = Tm4c_flash_page_unit_for_lc3::new(flash_unit);

swap_obj.write_primary(1532,0x0000);
swap_obj.write_primary(1533,0x0000);
swap_obj.write_primary(1534,0x0000);
swap_obj.write_primary(1535,0x0000);
swap_obj.write_primary(1536,0x3000);
swap_obj.write_primary(1537,0x0001);
swap_obj.write_primary(1538,0x0700);
swap_obj.write_primary(1539,0x0000);
swap_obj.write_primary(1540,0x0000);
swap_obj.write_primary(1541,0x0000);

    let sev = swap_obj.read_primary(12288);
        match sev{
       	Ok(out)=>{
       		let x = out;

       	}
       	_=>{
       		loop{}
       	}
       }
    let mut tm4c_mem = tm4c_lc3_memory{
    	tm4c_mem_obj: swap_obj,
    };

   // let sys = sc.constrain();
    let mut pwm_shim = pwm::PwmShim::new(pwm_req{
        //sysctl: sc,
        portb: portb,
        portd: portd,
        pwm0: pwm0,
        pwm1: pwm1,
    }, &sys.power_control);

    let mut timer_shim = timers::TimersShim::new(&sys.power_control, timer_req{timer0: t0, timer1: t1});

    let mut clock_req = clock::Tm4cClock::new(clock_req{timer: t2}, &sys.power_control);
    let x: PeripheralsStub;
    let peripherals = PeripheralSet::new(
        GpioStub,
        adc_shim,
        pwm_shim,
        timer_shim,
        clock_req,
        InputStub,
        OutputStub,
    );




        let mut interp = Interpreter::<tm4c_lc3_memory, 
        Peripheralstm4c>::new(
        tm4c_mem,
        peripherals,
        OwnedOrRef::<PeripheralInterruptFlags>::default(),
        [0 as Word; Reg::NUM_REGS],
        0x3000,
        MachineState::Halted,


    );
      let mut out = interp.get_register(Reg::R0);

  	  let mut pc = interp.get_pc();

       while (interp.get_pc() != 12288){
       pc = interp.get_pc();
       let word = interp.get_word(pc);
       match word {
           Ok(res) =>{
            let x = res;
           },
           _=> {
            let failed = 1;
           },
       }
        
        interp.step();


      }

      let mut a0=0;
      let mut a1=0;
      let mut a2=0;
     loop{
        interp.set_pc(12288);
         interp.set_register(Reg::R0, 0);
        while (interp.get_pc() != 12289){
          interp.step();
          pc = interp.get_pc();
        }       
        interp.set_pc(12289);
        interp.set_register(Reg::R0, 0);
        while (interp.get_pc() != 12290){
          interp.step();
          pc = interp.get_pc();
        }
        a0 = interp.get_register(Reg::R0);

        if (a0<187){
          interp.set_pc(12290);
          interp.set_register(Reg::R0, 0);
          interp.set_register(Reg::R1, 255);
          interp.set_register(Reg::R2, 110);
          while (interp.get_pc() != 12291){
          interp.step();
         }    
        }

        else{
          interp.set_pc(12291);
          interp.set_register(Reg::R0, 0);
          // interp.set_register(Reg::R1, 255);
          // interp.set_register(Reg::R2, 1);
          while (interp.get_pc() != 12292){
          interp.step();
         }    
        }

        interp.set_pc(12288);
         interp.set_register(Reg::R0, 1);
        while (interp.get_pc() != 12289){
          interp.step();
          pc = interp.get_pc();
        }  
        interp.set_pc(12289);
        interp.set_register(Reg::R0, 1);
        while (interp.get_pc() != 12290){
          interp.step();
        }
        a1 = interp.get_register(Reg::R0);

        if (a1<187){
          interp.set_pc(12290);
          interp.set_register(Reg::R0, 1);
          interp.set_register(Reg::R1, 255);
          interp.set_register(Reg::R2, 110);
          while (interp.get_pc() != 12291){
          interp.step();
         }    
        }

        else{
          interp.set_pc(12291);
          interp.set_register(Reg::R0, 1);
          // interp.set_register(Reg::R1, 255);
          // interp.set_register(Reg::R2, 1);
          while (interp.get_pc() != 12292){
          interp.step();
         }   
        }


     }

    loop{

       let came_here = 1;
    }
}


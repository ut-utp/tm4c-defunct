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
    Pwm, PwmPin, PwmPinArr, PwmSetDutyError, PwmSetPeriodError, PwmState,
};

use lc3_tm4c::peripherals_tm4c::pwm;
use lc3_tm4c::peripherals_tm4c::pwm::required_components;

use lc3_tm4c::peripherals_tm4c::Peripheralstm4c;

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
    let mut sc = p.SYSCTL;
    let mut sys = sc.constrain();
     sys.clock_setup.oscillator = hal::sysctl::Oscillator::Main(
        hal::sysctl::CrystalFrequency::_16mhz,
        hal::sysctl::SystemClock::UsePll(hal::sysctl::PllOutputFrequency::_80_00mhz),
    );
    let clocks = sys.clock_setup.freeze();
   // let mut portf = p.GPIO_PORTF;
   // let mut porte = p.GPIO_PORTE;
    let mut adc0 = p.ADC0;
    let mut adc1= p.ADC1;
    let mut pwm0 = p.PWM0;
    let mut pwm1 = p.PWM1;
    let mut flash = p.FLASH_CTRL;

    let mut portb = p.GPIO_PORTB;
    let mut portd = p.GPIO_PORTD;
    let mut porta = p.GPIO_PORTF;
    let mut porte = p.GPIO_PORTE;
    
	let mut flash_unit = flash::tm4c_flash_unit{
		flash_ctrl: flash,
	};


    let mut swap_obj = Tm4c_flash_page_unit_for_lc3::new(flash_unit);
    let mut tm4c_mem = tm4c_lc3_memory{
    	tm4c_mem_obj: swap_obj,
    };

   // let sys = sc.constrain();
    let mut pwm_shim = pwm::PwmShim::new(required_components {
        //sysctl: sc,
        portb: portb,
        portd: portd,
        pwm0: pwm0,
        pwm1: pwm1,
    }, &sys.power_control);


    let x: PeripheralsStub;
    let peripherals = PeripheralSet::new(
        GpioStub,
        AdcStub,
        pwm_shim,
        TimersStub,
        ClockStub,
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

     interp.set_word(0x3000, 0x1021);
     interp.set_word(0x3001, 0x1021);
     interp.set_word(0x3002, 0x1021);
     interp.set_word(0x3003, 0x1021);
     interp.set_word(0x3004, 0x1021);
     interp.set_word(0x3005, 0x1021);
     interp.set_pc(0x3000);

      interp.step();
      interp.step();
      interp.step();
      let out = interp.get_register(Reg::R0);
     let res = interp.get_word(0x3005).unwrap();
     

        // interp.set_word(0x3001, 1021);

        // interp.step();
    //let tot_free_secs = swap_obj.get_total_free_sectors();
    //swap_obj.read_swap(0x0000);
    // let mut u0 = p.UART0;
    // let mut u1 = p.UART1;
    // let mut porta = p.GPIO_PORTA.split(&sys.power_control);
    // let mut uart = hal::serial::Serial::uart0(
    //     u0,
    //     porta
    //         .pa1
    //         .into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
    //     porta
    //         .pa0
    //         .into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
    //     (),
    //     (),
    //     9_600_u32.bps(),
    //     hal::serial::NewlineMode::SwapLFtoCRLF,
    //     &clocks,
    //     &sys.power_control,
    // );
     
    loop{
     // 	for i in 0..2 {
    	// let out = swap_obj.write_primary(i, 4);
    	// match out{
    	// 	Ok(()) => {
    	// 		writeln!(uart,"Write success");

    	// 	},
    	// 	_=>{
    	// 		let y=5;
    	// 	}
    	// }
     // }

    	// for i in 0..2 {
    	// let out = swap_obj.read_primary(i);
    	// match out{
    	// 	Ok(word) => {
    	// 		writeln!(uart,"{}", word);

    	// 	},
    	// 	_=>{
    	// 		let y=5;
    	// 	}
    	// }
     // }
     // writeln!(uart,"HELLO!");
       //    for i in 0..100 {
       //       let data_bytes: [usize; 128] = [100usize; 128];
       //       flash_unit.Flash_ProgramData(0x00008000 + 512*i, data_bytes);  
       //   }
         
 

   

       //  for addr in (0x00008000..(0x00008000 + (512*100 as usize))).step_by(4) {
       //      let mut x = addr;
       //      let mut data = unsafe {read_volatile(x as (*const u32))};
       //      //writeln!(uart, "{}: [{:#x}] =  {:#x}", addr, addr, data);
       //      if(data!=100){
       //          loop{}
       //      }
       //      //data = data+1;
            
       // }
       // for i in 0u32..65536u32 {

       // 	let x = i as u16;
       // swap_obj.write_primary(x, x);


       // }

       // for i in 0u32..65536u32 {
       // 	let x = i as u16;
       // let out = swap_obj.read_primary(x);
       // match out{
       // 	Ok(out)=>{
       // 		let x = out;
       // 		if(x as u32 != i){
       // 			loop{}
       // 		}
       // 	}
       // 	_=>{
       // 		loop{}
       // 	}
       // }
       // }

       let came_here = 1;

    	
    }
}


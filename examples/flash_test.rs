#![no_main]
#![no_std]

extern crate panic_halt;
extern crate tm4c123x_hal as hal;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use hal::prelude::*;

use lc3_tm4c::peripherals_tm4c::flash;
use lc3_tm4c::peripherals_tm4c::flash::*;
use lc3_tm4c::persistent_data_management::flash::*;
use lc3_tm4c::persistent_data_management::page::Paging;
use lc3_tm4c::paging_impl::tm4c_flash_paging_config::*;
use lc3_tm4c::paging_impl::tm4c_flash_paging_config;
use core::ptr::read_volatile;
use core::fmt::Write;


#[entry]
fn main() -> ! {
let p = hal::Peripherals::take().unwrap();
    let mut sc = p.SYSCTL.constrain();
    sc.clock_setup.oscillator = hal::sysctl::Oscillator::Main(
        hal::sysctl::CrystalFrequency::_16mhz,
        hal::sysctl::SystemClock::UsePll(hal::sysctl::PllOutputFrequency::_80_00mhz),
    );
     let clocks = sc.clock_setup.freeze();
    // let mut porta = p.GPIO_PORTA.split(&sc.power_control);
    // let mut portb = p.GPIO_PORTB.split(&sc.power_control);
    // let mut u0 = p.UART0;
    // let mut u1 = p.UART1;
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
    //     1_500_000_u32.bps(),
    //     hal::serial::NewlineMode::SwapLFtoCRLF,
    //     &clocks,
    //     &sc.power_control,
    // );
    // writeln!(uart, "HELLO!");
 	  //  // let mut pwm1 = p.PWM1;
     let mut flash_unit = flash::tm4c_flash_unit{
            flash_ctrl: p.FLASH_CTRL,
     };
 	  //   //flash_unit.Flash_WriteWord(0x00008000, 124068);
 	  //   let addr: *const u32 = 0x00008000 as (*const u32);
 	  //   let mut result=0;
 	  //   unsafe{result = read_volatile(addr);}

 // Flash_Erase(FLASH);                               // erase  through 0x000083FC
 // Flash_Write(FLASH + 0, 0x10101010);               // write to location 0x00008000
 	    //adc_shim.set_state(Pin::A0, AdcState::Enabled);
    loop{

		 	
		//TEST 1
         flash_unit.Flash_EraseSector(0x00008000);
         let data_bytes: [usize; 128] = [100usize; 128];
        flash_unit.Flash_ProgramData(0x00008000, data_bytes);	



        for addr in (0x00008000..(0x00008000 + (512 as usize))).step_by(4) {
            let mut x = addr;
        	let mut data = unsafe {read_volatile(x as (*const u32))};
        	//writeln!(uart, "{}: [{:#x}] =  {:#x}", addr, addr, data);
            if(data!=100){
                loop{}
            }
        	//data = data+1;
            
       }


       //TEST 2
         for i in 0..100 {
             let data_bytes: [usize; 128] = [100usize; 128];
             flash_unit.Flash_ProgramData(0x00008000 + 512*i, data_bytes);  
         }
         
 



        for addr in (0x00008000..(0x00008000 + (512*100 as usize))).step_by(4) {
            let mut x = addr;
            let mut data = unsafe {read_volatile(x as (*const u32))};
            //writeln!(uart, "{}: [{:#x}] =  {:#x}", addr, addr, data);
            if(data!=100){
                loop{}
            }
            //data = data+1;
            
       }
       let pass_point = 1;

       //TEST 3



       //TEST 4


       //TEST 5



       //TEST 6



       //TEST 7


       //TEST 8



       //TEST 9




       //TEST 10



       //TEST 11
    }


		
}
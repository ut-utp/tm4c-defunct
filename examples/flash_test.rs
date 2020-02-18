#![no_main]
#![no_std]

extern crate panic_halt;
extern crate tm4c123x_hal as hal;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use hal::prelude::*;

use lc3_tm4c::peripherals_tm4c::flash;
use lc3_tm4c::peripherals_tm4c::flash::*;
use core::ptr::read_volatile;


#[entry]
fn main() -> ! {
 	    let p = hal::Peripherals::take().unwrap();
 	    //let mut porte = p.GPIO_PORTE;
 	    //let mut adc0 = p.ADC0;
 	    //let mut adc1= p.ADC1;
 	    let mut sc = p.SYSCTL.constrain();
 	   // let mut pwm1 = p.PWM1;
 	    let mut flash_unit = flash::tm4c_flash_unit{};
 	    flash_unit.TM4C_Flash_EraseSector(0x00008000);
 	    flash_unit.TM4C_Flash_WriteWord(0x00008000, 124068);

 	    let addr: *const u32 = 0x00008000 as (*const u32);
 	    let mut result=0;
 	    unsafe{result = read_volatile(addr);}

 // Flash_Erase(FLASH);                               // erase  through 0x000083FC
 // Flash_Write(FLASH + 0, 0x10101010);               // write to location 0x00008000
 	    //adc_shim.set_state(Pin::A0, AdcState::Enabled);
 	    loop{
 	   // adc_shim.read(Pin::A0);
 	}

		
}
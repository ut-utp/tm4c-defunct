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
    let mut sc = p.SYSCTL.constrain();
​
    sc.clock_setup.oscillator = hal::sysctl::Oscillator::Main(
        hal::sysctl::CrystalFrequency::_16mhz,
        hal::sysctl::SystemClock::UsePll(hal::sysctl::PllOutputFrequency::_80_00mhz),
    );
    let clocks = sc.clock_setup.freeze();
​
    let mut porta = p.GPIO_PORTA.split(&sc.power_control);
    let mut portb = p.GPIO_PORTB.split(&sc.power_control);
​
    let mut u0 = p.UART0;
    let mut u1 = p.UART1;
​
    let mut uart = hal::serial::Serial::uart0(
        u0,
        porta
            .pa1
            .into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
        porta
            .pa0
            .into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
        (),
        (),
        1_500_000_u32.bps(),
        hal::serial::NewlineMode::SwapLFtoCRLF,
        &clocks,
        &sc.power_control,
    );
​
    writeln!(uart, "HELLO!");

 	   // let mut pwm1 = p.PWM1;
 	    let mut flash_unit = flash::tm4c_flash_unit{};
 	    flash_unit.TM4C_Flash_EraseSector(0x00008000);
 	    let data_bytes: [usize; flash::MAX_WRITABLE_WORDS] = [100usize; flash::MAX_WRITABLE_WORDS];

 	    // for i in 0..flash::MAX_WRITABLE_WORDS{
 	    // 	data_bytes[i] = 1234;
 	    // }
 	    flash_unit.TM4C_Flash_ProgramData(0x00008000, data_bytes);
 	    //flash_unit.TM4C_Flash_WriteWord(0x00008000, 124068);

 	    let addr: *const u32 = 0x00008000 as (*const u32);
 	    let mut result=0;
 	    unsafe{result = read_volatile(addr);}

 // Flash_Erase(FLASH);                               // erase  through 0x000083FC
 // Flash_Write(FLASH + 0, 0x10101010);               // write to location 0x00008000
 	    //adc_shim.set_state(Pin::A0, AdcState::Enabled);
    loop{

		 	
		    //...
		
        for addr in (0x00008000..(0x00008000 + (MAX_WRITABLE_WORDS as usize))).step_by(4) {

        	let mut data = unsafe {read_volatile(addr as (*const u32))};
        	writeln!(uart, "{}: [{:#x}] =  {:#x}", addr, addr, data);
        	//data = data+1;
            
       }
    }


		
}
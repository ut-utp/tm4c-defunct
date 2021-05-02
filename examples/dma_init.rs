#![no_main]
#![no_std]

extern crate panic_halt;
extern crate tm4c123x_hal as hal;
use cortex_m_rt::entry;
use hal::prelude::*;

#[entry]
fn main() -> ! {


 	    let p = hal::Peripherals::take().unwrap();
 	    let mut channel_control: [u8; 1024] = [0; 1024];
 	    let mut sys_control = p.SYSCTL;

 	    let mut dma = p.UDMA;

 	    let channel_base_addr = &channel_control as *const u8;
 	    sys_control.rcgcdma.write(|w| unsafe{w.bits(1)});
 	    dma.cfg.write(|w| unsafe{w.bits(1)});
 	    dma.ctlbase.write(|w| unsafe{w.bits(channel_base_addr as u32)});

	loop{}
}
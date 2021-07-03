extern crate panic_halt;
extern crate tm4c123x_hal as hal;
extern crate tm4c123x;
extern crate typenum;
use cortex_m_rt::entry;
use hal::prelude::*;
use tm4c123x::generic::Reg;
use lc3_device_support::rpc::transport::uart_dma::*;

use core::cell::UnsafeCell;
use cortex_m::interrupt as int;

const tm4c_dma_control_entries: usize = 4;  // 4 32 it entries for each channel
const tm4c_dma_uart0_rx_control_channel: usize = 8;
const tm4c_dma_uart0_tx_control_channel: usize = 9;

const tm4c_dma_uart0_rx_control_index: usize = tm4c_dma_uart0_rx_control_channel*tm4c_dma_control_entries;
const tm4c_dma_uart0_tx_control_index: usize = tm4c_dma_uart0_tx_control_channel*tm4c_dma_control_entries;

struct DMAStatus(UnsafeCell<u8>);

#[derive(Copy, Clone)]
#[repr(align(1024))] //control structure must be 1024 byte aligned according to datasheet.
struct dma_control_structure([u32; 256]);

impl DMAStatus {
    pub fn set_complete(&self, _dma_ind: &int::CriticalSection) {
        // By requiring a CriticalSection be passed in, we know we must
        // be operating inside a CriticalSection, and so can confidently
        // use this unsafe block (required to call UnsafeCell::get).
        unsafe { *self.0.get() = 1 };
    }

    pub fn set_in_progress(&self, _dma_ind: &int::CriticalSection) {
        unsafe { *self.0.get() = 0 };
    }

    pub fn read_status(&self, _dma_ind: &int::CriticalSection) -> u8 {
        unsafe {*self.0.get()}
    }
}

const DMA_COMPLETE_INDICATOR: DMAStatus = DMAStatus(UnsafeCell::new(0));


// Should eventually have a dma struct for all peripherals and delegate to peripherals that need to use dma
pub struct tm4c_uart_dma_ctrl<'a>{
	power_ctrl: &'a mut tm4c123x::SYSCTL,

	//TODO: abstract this control structure away from here. Better to just have a global variable with some safe features here. 
	//That way, it's guaranteed to be static lifetime and put in .bss or data segment. Seems strange to initialize a byte aligned
	//structure on stack anyway.
	channel_control: dma_control_structure, 
	device_dma: tm4c123x::UDMA,

	// add uart control fields

}

impl<'a> tm4c_uart_dma_ctrl <'a> {
	pub fn new(dma_component: tm4c123x::UDMA, power: &'a mut tm4c123x::SYSCTL) -> Self{

        let control_struct = dma_control_structure([0; 256]);
        
		Self{
			power_ctrl: power,
			channel_control: control_struct,
			device_dma: dma_component,
		}

	}
}

impl<'a> DmaChannel for tm4c_uart_dma_ctrl <'a>{

    fn dma_device_init(&mut self){
 	    let channel_base_addr = &self.channel_control.0 as *const u32;

 	    self.power_ctrl.rcgcdma.write(|w| unsafe{w.bits(1)});
 	    self.device_dma.cfg.write(|w| unsafe{w.bits(1)});
 	    self.device_dma.ctlbase.write(|w| unsafe{w.bits(channel_base_addr as u32)});
        self.device_dma.reqmaskclr.write(|w| unsafe{w.bits(0x100)});
        self.device_dma.enaset.write(|w| unsafe{w.bits(0x100)});

 	    let mut uart_rx_control_slice: &mut [u32] = &mut self.channel_control.0[tm4c_dma_uart0_rx_control_index..tm4c_dma_uart0_rx_control_index+4];

 	     //uart_rx_control_slice[0] = unsafe{&((*hal::serial::UART0::ptr()).dr) as *const Reg<u32, hal::tm4c123x::uart0::_DR> as u32}; // Works but is it necessary? Better way to get a raw pointer to uart data register?
 	     uart_rx_control_slice[0] = 0x4000_c000 as *const u32 as u32;  // index entry of the control struct is source address (UART data register in this case)


 	     //index 2 is DMA channel control struct. Check datasheet page 611 for details. Here it represents dest addr increment by 1 byte; src addr fixed; 1 byte each; basic mode. Yet to complete and decide this
 	     //also create some abstractions and maybe a little dma API for tm4c to make this more generic rather than harcoding what we want it to do
 	     uart_rx_control_slice[2] = 0x0c00_0000 | 0x0311;

         let uart0_temp = unsafe{&*tm4c123x::UART0::ptr()}; //Can fix this by maybe allowing dma own and consume uart? It is tricky though to avoid some stealing due to these cross linked peripherals. 
                                                            //STM dma impl also uses these hacks. Revisit this later. There could be cleaner ways to do it. 
         uart0_temp.dmactl.write(|w| unsafe{w.bits(1)});
         uart0_temp.im.write(|w| unsafe{w.bits(0x10)});

    }


    // No need of these 2 functions fr uart specifi generic dma control
    fn dma_set_destination_address(&mut self, address: usize) {

        let mut uart_rx_control_slice: &mut [u32] = &mut self.channel_control.0[tm4c_dma_uart0_rx_control_index..tm4c_dma_uart0_rx_control_index+4];
        uart_rx_control_slice[1] = (address as u32);
    }

    fn dma_set_source_address(&mut self, address: usize){
        unimplemented!()
    }

    // determined by XFERSIZE, ARBSIZE bits
    fn dma_set_transfer_length(&mut self, len: usize){
        let mut uart_rx_control_slice: &mut [u32] = &mut self.channel_control.0[tm4c_dma_uart0_rx_control_index..tm4c_dma_uart0_rx_control_index+4];
        uart_rx_control_slice[1] = uart_rx_control_slice[1] + len as u32 - 1;
    }

    fn dma_start(&mut self){
       int::free(|dma_ind| DMA_COMPLETE_INDICATOR.set_in_progress(dma_ind));
       //set the bit to start burst transaction here, enable arbitration on uart with high priority (nothing else uses dma)
    }

    fn dma_stop(&mut self){
        unimplemented!()
    }

    fn dma_in_progress(&mut self) -> bool{
    	let mut dma_in_prog: bool = true;
        let status: u8 = int::free(|dma_ind| DMA_COMPLETE_INDICATOR.read_status(dma_ind));
        if(status == 1){
        	dma_in_prog = false;
        }
        else{
        	dma_in_prog = true;
        }
        dma_in_prog
    }

    fn dma_num_bytes_transferred(&mut self) -> usize{
        unimplemented!()
    }

    //Add an other method here to read the data and return on consumer side. The method checks the completion status and commits in bbqueue the number of bytes dma finished transferring
}

use cortex_m_rt_macros::interrupt;
use tm4c123x::Interrupt as interrupt;


// #[interrupt]
// fn UDMA(){
// }


// #[interrupt]
// fn UART0(){

// 	//First check the bit that triggered this interrupt. there is a bit that's set when dma transaction is complete and dma invokes uart vector,
//     //TODO: Instead of this, safely share the dma peripheral between background and foreground threads as described 
// 	unsafe{
// 		let mut dma = &*tm4c123x::UDMA::ptr();
// 		let bits = dma.chis.read().bits();
// 		if((bits & 0x100) == 0x100){
// 			int::free(|dma_ind| DMA_COMPLETE_INDICATOR.set_complete(dma_ind));
// 		}
// 	}
// }
extern crate panic_halt;
extern crate tm4c123x_hal as hal;
extern crate tm4c123x;
extern crate typenum;
use cortex_m_rt::entry;
use hal::prelude::*;
use bbqueue::{BBBuffer, GrantR, GrantW, ConstBBBuffer, Consumer, Producer, consts::*};
use tm4c123x::generic::Reg;
use crate::peripherals_generic::dma as gen_dma;

use core::cell::UnsafeCell;
use cortex_m::interrupt as int;

const tm4c_dma_control_entries: usize = 4;  // 4 32 it entries for each channel
const tm4c_dma_uart0_rx_control_channel: usize = 8;
const tm4c_dma_uart0_tx_control_channel: usize = 9;

const tm4c_dma_uart0_rx_control_index: usize = tm4c_dma_uart0_rx_control_channel*tm4c_dma_control_entries;
const tm4c_dma_uart0_tx_control_index: usize = tm4c_dma_uart0_tx_control_channel*tm4c_dma_control_entries;

static rx_buffer: BBBuffer<U64> = BBBuffer( ConstBBBuffer::new() );
static tx_buffer: BBBuffer<U64> = BBBuffer( ConstBBBuffer::new() );

struct DMAStatus(UnsafeCell<u8>);

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
	channel_control: [u32; 256],
	device_dma: tm4c123x::UDMA,

	rx_prod: Producer<'a, U64>,
	rx_cons: Consumer<'a, U64>,
	tx_prod: Producer<'a, U64>,
	tx_cons: Consumer<'a, U64>,
	// add uart control fields

}

impl<'a> tm4c_uart_dma_ctrl <'a> {
	pub fn new(dma_component: tm4c123x::UDMA, power: &'a mut tm4c123x::SYSCTL) -> Self{

		let (mut rx_p, rx_c) = rx_buffer.try_split().unwrap();
        let (tx_p, tx_c) = tx_buffer.try_split().unwrap();
        
		Self{
			power_ctrl: power,
			channel_control: [0; 256],
			device_dma: dma_component,
			rx_prod: rx_p,
			rx_cons: rx_c,
			tx_prod: tx_p,
			tx_cons: tx_c,
		}

	}
}

impl<'a> gen_dma::DmaChannel for tm4c_uart_dma_ctrl <'a>{

    fn dma_device_init(&mut self){
 	    let channel_base_addr = &self.channel_control as *const u32;

 	    let mut rx_grant = self.rx_prod.grant_exact(32).unwrap();

 	    self.power_ctrl.rcgcdma.write(|w| unsafe{w.bits(1)});
 	    self.device_dma.cfg.write(|w| unsafe{w.bits(1)});
 	    self.device_dma.ctlbase.write(|w| unsafe{w.bits(channel_base_addr as u32)});

 	    let mut uart_rx_control_slice: &mut [u32] = &mut self.channel_control[tm4c_dma_uart0_rx_control_index..tm4c_dma_uart0_rx_control_index+4];

 	     uart_rx_control_slice[0] = unsafe{&((*hal::serial::UART0::ptr()).dr) as *const Reg<u32, hal::tm4c123x::uart0::_DR> as u32}; // Works but is it necessary? Better way to get a raw pointer to uart data register?
 	     uart_rx_control_slice[0] = 0x4000_c000 as *const u32 as u32;  // index entry of the control struct is source address (UART data register in this case)
 	     uart_rx_control_slice[1] = rx_grant.buf().as_ptr() as u32;    // index entry 1 is destination address. Point to bbqueue buffer


 	     //index 2 is DMA channel control struct. Check datasheet page 611 for details. Here it represents dest addr increment by 1 byte; src addr fixed; 1 byte each; basic mode. Yet to complete and decide this
 	     uart_rx_control_slice[2] = 0x00 | (0xA << 14) | (0x1 << 29);

    }


    // No need of these 2 functions fr uart specifi generic dma control
    fn dma_set_destination_address(&mut self, address: usize, inc: bool) {
         unimplemented!()
        // ..
    }

    fn dma_set_source_address(&mut self, address: usize, inc: bool){
        unimplemented!()
    }

    // determined by XFERSIZE, ARBSIZE bits
    fn dma_set_transfer_length(&mut self, len: usize){
        unimplemented!()
    }

    fn dma_start(&mut self){
       int::free(|dma_ind| DMA_COMPLETE_INDICATOR.set_in_progress(dma_ind));
       //set the bit to start burst transaction here, enable arbitration on uart with high priority (nothing else uses dma)
    }

    fn dma_stop(&mut self){
        unimplemented!()
    }

    fn dma_in_progress() -> bool{
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

    //Add an other method here to read the data and return on consumer side. The method checks the completion status and commits in bbqueue the number of bytes dma finished transferring
}

use cortex_m_rt_macros::interrupt;
use tm4c123x::Interrupt as interrupt;


#[interrupt]
fn UDMA(){
}


#[interrupt]
fn UART0(){

	//First check the bit that triggered this interrupt. there is a bit that's set when dma transaction is complete and dma invokes uart vector,
    //TODO: Instead of this, safely share the dma peripheral between background and foreground threads as described 
	unsafe{
		let mut dma = &*tm4c123x::UDMA::ptr();
		let bits = dma.chis.read().bits();
		if((bits & 0x100) == 0x100){
			int::free(|dma_ind| DMA_COMPLETE_INDICATOR.set_complete(dma_ind));
		}
	}
}
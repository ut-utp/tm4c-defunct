use crate::persistent_data_management::disk;
use crate::persistent_data_management::disk::*;
use crate::persistent_data_management::page;
use crate::persistent_data_management::flash::{Flash, status_error_codes};
use crate::peripherals_tm4c::flash;
use crate::peripherals_tm4c::flash::*;
use lc3_isa::{
    Addr, Instruction, Word,
};
use crate::persistent_data_management::page::{Paging, SwapError};

const PRIMARY_START_SECTOR: u32 = 80;   //This will be given a value once code is frozen
const PRIMARY_NUM_SECTORS:  u32 = 128;
const SWAP_START_SECOTR:    u32 = 300;
const SWAP_SIZE        :    u32 = 100;


pub const total_lc3_address_space_sectors: u32  = 256;
pub const total_meta_data_sectors: u32 =   4;

pub struct Tm4c_flash_page_unit_for_lc3{
	page_table:        [u32; SWAP_SIZE as usize],
	swap_start_Addr:   u32,
	swap_end_addr:     u32,
    num_swap_pages:    u32,
	tm4c_flash_unit:   flash::tm4c_flash_unit,

}



impl Paging for Tm4c_flash_page_unit_for_lc3{
	type storage = flash::tm4c_flash_unit;
	type device_addr = Addr;
	type device_word = Word;

    fn get_total_free_sectors(&mut self)->u32{

    	let b0: u32 = self.tm4c_flash_unit.flash_ctrl.fmppe0.read().bits();
    	let b1: u32 = self.tm4c_flash_unit.flash_ctrl.fmppe1.read().bits();
    	let b2: u32 = self.tm4c_flash_unit.flash_ctrl.fmppe2.read().bits();
    	let b3: u32 = self.tm4c_flash_unit.flash_ctrl.fmppe3.read().bits();

    	let free_sectors_b0 = u32::count_ones(b0);
    	let free_sectors_b1 = u32::count_ones(b1);
    	let free_sectors_b2 = u32::count_ones(b2);
    	let free_sectors_b3 = u32::count_ones(b3);

    	4*(free_sectors_b0+free_sectors_b1+free_sectors_b2+free_sectors_b3) - total_meta_data_sectors
    }
	fn set_primary_partition(&mut self, num_sectors: u32){

	}
	fn set_swap_spartition(&mut self, num_sectors: u32){}

	fn initialize(&mut self) -> Result<(), SwapError>{
	Ok(())
}

    fn read_swap(&self, addr: Addr) -> Result<Word, SwapError>{
/*    	let root_sec: [u32; 128] = [0; 128];
    	for i in 0..128 {
    		let res = self.tm4c_flash_unit.Flash_ReadData((ROOT_METADATA_SECTOR*SECTOR_SIZE) as usize + 4*i as usize, 1).unwrap();
    		match(res){
    			status_error_codes::ARM_DRIVER_OK_READ(inp) =>{
    				root_sec[i] = inp as u32;
    			}
    			_=>{

    			}
    		}
    	}*/
        let mut flag = 300;
        let sector = lc3_addr_to_sec_map(addr);
        let mut word_at_addr: Word = 0;
        for i in 0..self.num_swap_pages {
            if(self.page_table[i as usize]==sector){
                flag = i;
                break;
            }
        }
        if(flag != 300){
           // Err((SwapError::AddressOutOfRange))
        
        let physical_sector = SWAP_START_SECOTR+flag;
        let physical_addr   = (physical_sector*512 as u32) + ((addr as u32)%512 as u32);
        let mut word = self.tm4c_flash_unit.Flash_ReadData(physical_addr as u32, 1);
        
        match word{
            status_error_codes::ARM_DRIVER_OK_READ(inp) =>{
                if(addr&1 == 1){
                    word_at_addr = inp as Word;
                }
                else {
                    word_at_addr = (inp >> 16) as Word;
                }
                flag = 1;
            }
            _=>{
                flag= 300;
            }                           
        }
       }
        if(flag == 300){
            Err(SwapError::AddressOutOfRange)
        }
        else{
            Ok((word_at_addr))
        }

       // Ok((4))

     	
    	//let result = self.tm4c_flash_unit.flash_ctrl.Flash_ReadData();
    }
    fn write_swap(&mut self, addr: Addr, data: Word) -> Result<(), SwapError>{
        let mut flag = 300;
        let sector = lc3_addr_to_sec_map(addr);
       // let mut word_at_addr: Word = 0;
        for i in 0..self.num_swap_pages {
            if(self.page_table[i as usize]==sector){
                flag = i;
                break;
            }
        }
        if(flag != 300){
           // Err((SwapError::AddressOutOfRange))
        
        let physical_sector = SWAP_START_SECOTR+flag;
        let physical_addr   = (physical_sector*512 as u32) + ((addr as u32)%512 as u32);
        let mut word = self.tm4c_flash_unit.Flash_ReadData(physical_addr as u32, 1);
        
        match word{
            status_error_codes::ARM_DRIVER_OK_READ(mut inp) =>{
                if(addr&1 == 1){
                    inp = (inp&0xFFFF0000)+data as u32;
                }
                else {
                    inp = (inp&0x0000FFFF)+(((data as u32)<<16)&0xFFFF0000);
                }
                self.tm4c_flash_unit.Flash_WriteWord(physical_addr as u32, inp);
                flag = 1;
            }
            _=>{
                flag = 300;
            }                           
        }
       }
        if(flag == 300){
            Err(SwapError::AddressOutOfRange)
        }
        else{
            Ok(())
        }

    }
    fn read_primary(&self, addr: Addr) -> Result<Word, SwapError>{
                let mut flag = 300;
        let sector = lc3_addr_to_sec_map(addr);
        let mut word_at_addr: Word = 0;

           // Err((SwapError::AddressOutOfRange))
        
        let physical_sector = PRIMARY_START_SECTOR+sector;
        let mut physical_addr   = (physical_sector*512 as u32) + (((addr as u32)*2 as u32)%512 as u32);
        if(physical_addr&0x01==0 && physical_addr%4!=0){
            physical_addr -= 2;
        }
        let mut word = self.tm4c_flash_unit.Flash_ReadData(physical_addr as u32, 1);
        
        match word{
            status_error_codes::ARM_DRIVER_OK_READ(inp) =>{
                if(addr&1 == 1){
                    word_at_addr = inp as Word;
                }
                else {
                    word_at_addr = (inp >> 16) as Word;
                }
                flag = 1;
            }
            _=>{
                flag= 300;
            }                           
        }
       
        if(flag == 300){
            Err(SwapError::AddressOutOfRange)
        }
        else{
            Ok((word_at_addr))
        }

    }
    fn write_primary(&mut self, addr: Addr, data: Word) -> Result<(), SwapError>{
        let mut flag = 300;
        let sector = lc3_addr_to_sec_map(addr);
       // let mut word_at_addr: Word = 0;
           // Err((SwapError::AddressOutOfRange))
        
        let physical_sector = PRIMARY_START_SECTOR+sector;
        let mut physical_addr   = (physical_sector*512 as u32) + (((addr as u32)*2 as u32)%512 as u32);
        if(physical_addr&0x01==0 && physical_addr%4!=0){
            physical_addr -= 2;
        }
        let mut word = self.tm4c_flash_unit.Flash_ReadData(physical_addr as u32, 1);
        
        match word{
            status_error_codes::ARM_DRIVER_OK_READ(mut inp) =>{
                if(addr&1 == 1){
                    inp = (inp&0xFFFF0000)+data as u32;
                }
                else {
                    inp = (inp&0x0000FFFF)+(((data as u32)<<16)&0xFFFF0000);
                }
                self.tm4c_flash_unit.Flash_WriteWord(physical_addr as u32, inp);
                flag = 1;
            }
            _=>{
                flag = 300;
            }                           
        }
       
        if(flag == 300){
            Err(SwapError::AddressOutOfRange)
        }
        else{
            Ok(())
        }
    }
    fn commit_changes(&mut self, addr: Addr)-> Result<(), SwapError>{
    	unimplemented!()
    }
}



impl Tm4c_flash_page_unit_for_lc3{
	pub fn new(tm4c_flash_unit: flash::tm4c_flash_unit) -> Self{
		Tm4c_flash_page_unit_for_lc3{
            page_table:        [0; SWAP_SIZE as usize],
            swap_start_Addr:   0,
            swap_end_addr:     0,
            num_swap_pages:    0,
            tm4c_flash_unit:   tm4c_flash_unit,           
        }
	}
}
fn get_bit_at(input: u32, n: u8) -> bool {
        input & (1 << n) != 0
}

fn swap_sector_contains_address(input: Word) -> bool {
        true
}

fn lc3_addr_to_sec_map(input: Word) -> u32{
    let offset = (((input as u32)*2) - ((input as u32)*2)%512) as u32;
    offset/512
}
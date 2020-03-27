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

const ROOT_METADATA_SECTOR: u32 = 2;   //This will be given a value once code is frozen
const SECTOR_SIZE:      u32 =    512;


pub const total_lc3_address_space_sectors: u32  = 256;
pub const total_meta_data_sectors: u32 =   4;

pub struct Tm4c_flash_page_unit_for_lc3{
	page_table_sector: u32,
	swap_start_Addr:   u32,
	swap_end_addr:     u32,
	tm4c_flash_unit:   flash::tm4c_flash_unit,

}



impl Paging for Tm4c_flash_page_unit_for_lc3{
	type storage = flash::tm4c_flash_unit;
	type device_addr = Addr;
	type device_word = Word;

    fn get_total_free_sectors(&mut self)->u32{

    	let b0: u32 = self.tm4c_flash_unit.flash_ctrl.fmpre0.read().bits();
    	let b1: u32 = self.tm4c_flash_unit.flash_ctrl.fmpre1.read().bits();
    	let b2: u32 = self.tm4c_flash_unit.flash_ctrl.fmpre2.read().bits();
    	let b3: u32 = self.tm4c_flash_unit.flash_ctrl.fmpre3.read().bits();

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
		if( total_lc3_address_space_sectors < self.get_total_free_sectors()){

			let metadata: [u32; 128] = [0; 128];
			//fn initialize(&mut self, num_primary_sectors: u32, num_swap_sectors: u32);
			let out =self.tm4c_flash_unit.Flash_EraseSector(ROOT_METADATA_SECTOR*(SECTOR_SIZE));

			let mut free_sector_table: [u16; 512] = [0; 512];

	    	let b0: u32 = self.tm4c_flash_unit.flash_ctrl.fmpre0.read().bits();
	    	let b1: u32 = self.tm4c_flash_unit.flash_ctrl.fmpre1.read().bits();
	    	let b2: u32 = self.tm4c_flash_unit.flash_ctrl.fmpre2.read().bits();
	    	let b3: u32 = self.tm4c_flash_unit.flash_ctrl.fmpre3.read().bits();

	    	let mut pos: usize = 0;
	    	for i in 0..32 {

	    		if(get_bit_at(b0, i)){
	    			free_sector_table[pos] = (i*4) as u16;
	    			pos += 1;
	    			free_sector_table[pos] = (i*4 + 1) as u16;
	    			pos += 1;
	    			free_sector_table[pos] = (i*4 + 2) as u16;
	    			pos += 1;
	    			free_sector_table[pos] = (i*4 + 3) as u16;
	    			pos += 1;
	    		}
	    	}

	    	for i in 0..32 {

	    		if(get_bit_at(b1, i)){
	    			free_sector_table[pos] = (i*4 + 128) as u16;
	    			pos += 1;
	    			free_sector_table[pos] = (i*4 + 1 + 128) as u16;
	    			pos += 1;
	    			free_sector_table[pos] = (i*4 + 2 + 128) as u16;
	    			pos += 1;
	    			free_sector_table[pos] = (i*4 + 3 + 128) as u16;
	    			pos += 1;
	    		}
	    	}

	    	for i in 0..32 {

	    		if(get_bit_at(b2, i)){
	    			free_sector_table[pos] = (i*4 + 256) as u16;
	    			pos += 1;
	    			free_sector_table[pos] = (i*4 + 1 + 256) as u16;
	    			pos += 1;
	    			free_sector_table[pos] = (i*4 + 2 + 256) as u16;
	    			pos += 1;
	    			free_sector_table[pos] = (i*4 + 3 + 256) as u16;
	    			pos += 1;
	    		}
	    	}
	    	for i in 0..32 {

	    		if(get_bit_at(b3, i)){
	    			free_sector_table[pos] = (i*4 + 384) as u16;
	    			pos += 1;
	    			free_sector_table[pos] = (i*4 + 1 + 384) as u16;
	    			pos += 1;
	    			free_sector_table[pos] = (i*4 + 2 + 384) as u16;
	    			pos += 1;
	    			free_sector_table[pos] = (i*4 + 3 + 384) as u16;
	    			pos += 1;
	    		}
	    	}

	    	//let p_sec =self.tm4c_flash_unit.Flash_EraseSector(ROOT_METADATA_SECTOR*(SECTOR_SIZE));
			//metadata[0] = 

			for i in 0..64 {
				//metadata[i] = 0;
			}

		// }
		// else{

		// }
		Ok(())
	}
	else{
		Err(SwapError::Insufficient_free_space)
	}
}

    fn read_swap(&mut self, addr: Addr) -> Result<Word, SwapError>{
    	Ok((4))
    	//let result = self.tm4c_flash_unit.flash_ctrl.Flash_ReadData();
    }
    fn write_swap(&mut self, addr: Addr, data: Word) -> Result<Word, SwapError>{Ok(4)}
    fn read_primary(&mut self, addr: Addr) -> Result<(), SwapError>{Ok(())}
    fn write_primary(&mut self, data: Word) -> Result<(), SwapError>{Ok(())}
    fn commit_changes(&mut self, addr: Addr)-> Result<(), SwapError>{
    	Ok(())
    }
}

// impl Default for Tm4c_flash_page_unit{
// 	fn default() -> Self{
// 		unimplemented!()
// 	}
// }
fn get_bit_at(input: u32, n: u8) -> bool {
        input & (1 << n) != 0
}
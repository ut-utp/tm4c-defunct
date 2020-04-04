use lc3_traits::memory;
use lc3_traits::control::metadata::ProgramMetadata;
use lc3_traits::control::load::{PageIndex, PAGE_SIZE_IN_WORDS};

use lc3_isa::{Addr, Word};

use core::ops::{Index, IndexMut};
use crate::persistent_data_management::page::{Paging, SwapError};

pub struct tm4c_lc3_memory{

	pub tm4c_mem_obj: crate::paging_impl::tm4c_flash_paging_config::Tm4c_flash_page_unit_for_lc3,
	//word_idx: Word,
}



// INdexing makes no sense in device implementation
// What would it mean to say self[addr] = word on the tm4c?!
// It could fail in so many ways, and moreover, it's supposed to write to 
// persistent storage not just some array in RAM;
// Leaving Indexing unimplemented for now
// Only call read_Word and write_word instead

// One way I can think of that would make this work though is via paging on RAM
// i.e. when indexing, trigger a load of that page onto RAM and just modify
// that corresponding element of array in RAM. But this involves a more sophisticated paging
// that involves caching, and must handle power failures. I'll do this some day
// but for now only read and witeword functions must be used. No indexing support.
impl IndexMut<Addr> for tm4c_lc3_memory{

	fn index_mut(&mut self, index: Addr) -> &mut Self::Output{
		 // let out = self.tm4c_mem_obj.read_swap(index);
		 // let mut out_word: Word = 0xFFFF;
		 // match out{
		 // 	Ok((word)) =>{
		 // 		out_word = word;
		 // 	},
		 // 	Err(_) =>{
		 // 	}
		 // }
		 // self.word_idx = out_word;
		 // &mut self.word_idx
		 unimplemented!()

	}
}

impl Index<Addr> for tm4c_lc3_memory{
	type Output = Word;
	fn index(&self, index: Addr) -> &Word{
		 // let out = self.tm4c_mem_obj.read_swap(index);
		 // let mut out_word: Word = 0xFFFF;
		 // match out{
		 // 	Ok((word)) =>{
		 // 		out_word = word;
		 // 	},
		 // 	Err(_) =>{
		 // 	}
		 // }
		 // 
		 unimplemented!()
	}
}

impl memory::Memory for tm4c_lc3_memory{
    fn read_word(&self, addr: Addr) -> Word {
		 let out = self.tm4c_mem_obj.read_primary(addr);
		 let mut out_word: Word = 0xFFFF;
		 match out{
		 	Ok((word)) =>{
		 		out_word = word;
		 	},
		 	Err(_) =>{   
		 		// Should actually block if read fails so that host control
		 		// doesn't get a response back and it knows something went wrong
		 	}
		 }
		 out_word
		 
    }

    fn write_word(&mut self, addr: Addr, word: Word) {
		 let out = self.tm4c_mem_obj.write_primary(addr, word);
		 match out{
		 	Ok(()) =>{
		 		let res = 1;
		 	},
		 	Err(_) =>{   
		 		// Again, should block if read fails so that host control
		 		// doesn't get a response back and it knows something went wrong
		 	}
		 }
		 //out_word
    }

    fn commit_page(&mut self, page_idx: PageIndex, page: &[Word; PAGE_SIZE_IN_WORDS as usize]){unimplemented!()}
    fn reset(&mut self){}

    fn get_program_metadata(&self) -> ProgramMetadata{unimplemented!()}
    fn set_program_metadata(&mut self, metadata: ProgramMetadata){unimplemented!()}
}
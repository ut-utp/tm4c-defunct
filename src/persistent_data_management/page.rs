
use lc3_isa::{
    Addr, Instruction, Word,
};
use crate::persistent_data_management::flash::Flash;

pub enum SwapError {
	PrimaryPartitionUninitialized,
    SwapPartitionUninitialized,
    AddressOutOfRange,
    Insufficient_free_space,
}

pub trait Paging{

	type storage:Flash+Sized;
	type device_addr;
    type device_word;

    fn get_total_free_sectors(&mut self)->u32;
	fn set_primary_partition(&mut self, num_sectors: u32);
	fn set_swap_spartition(&mut self, num_sectors: u32);
    fn initialize(&mut self) -> Result<(), SwapError>;
    fn read_swap(&self, addr: Self::device_addr) -> Result<Self::device_word, SwapError>;
    fn write_swap(&mut self, addr: Self::device_addr, data: Self::device_word) -> Result<(), SwapError>;
    fn read_primary(&mut self, addr: Self::device_addr) -> Result<(Word), SwapError>;
    fn write_primary(&mut self, addr: Self::device_addr, data: Self::device_word) -> Result<(), SwapError>;
    fn commit_changes(&mut self, addr: Self::device_addr)-> Result<(), SwapError>;
}
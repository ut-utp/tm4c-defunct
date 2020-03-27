
pub const MAX_WRITABLE_WORDS:   usize = 128;

pub enum status_error_codes{
	ARM_DRIVER_OK_READ(u32),
    ARM_DRIVER_OK_WRITE,
    ARM_DRIVER_OK_ERASE,
    ARM_DRIVER_ERROR,
    ARM_DRIVER_ERROR_BUSY,
    ARM_DRIVER_ERROR_TIMEOUT,
    ARM_DRIVER_ERROR_UNSUPPORTED,
    ARM_DRIVER_ERROR_PARAMETER,
    ARM_DRIVER_ERROR_SPECIFIC,

}

pub trait Flash {

	fn Flash_Initialize(&mut self) -> status_error_codes;
	fn Flash_Uninitialize(&mut self) -> status_error_codes;
	fn Flash_ReadData(&mut self, addr: u32, num_items: u8) -> status_error_codes;
  	fn Flash_WriteWord(&mut self, addr: u32, data: u32)-> status_error_codes;
	fn Flash_ProgramData(&mut self, addr: u32, data: [usize; MAX_WRITABLE_WORDS])-> status_error_codes;
	fn Flash_EraseSector(&mut self, addr: u32) -> status_error_codes;
	fn Flash_EraseChip(&mut self) -> status_error_codes;
	fn Flash_GetStatus(&mut self);
	fn Flash_GetInfo(&mut self);

}
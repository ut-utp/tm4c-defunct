

// struct ARM_FLASH_INFO
// ARM_FLASH_SECTOR *	sector_info	Sector layout information (NULL=Uniform sectors)
// uint32_t	sector_count	Number of sectors.
// uint32_t	sector_size	Uniform sector size in bytes (0=sector_info used)
// uint32_t	page_size	Optimal programming page size in bytes.
// uint32_t	program_unit	Smallest programmable unit in bytes.
// uint8_t	erased_value	Contents of erased memory (usually 0xFF)
// uint8_t	reserved[3]	Reserved (must be zero)

pub const MAX_READABLE_WORDS: usize = 512;
pub const MAX_WRITABLE_WORDS: usize = 512;

pub struct TM4C_FLASH_STATUS{
	busy: u8,
	error: u8,
	reservd: u32,
}

pub struct TM4C_FLASH_SECTOR{
	start: u32,
	end: u32,
}

pub struct TM4C_FLASH_INFO{

//Data Fields
flash_sector: TM4C_FLASH_SECTOR,// *	sector_info	Sector layout information (NULL=Uniform sectors)
sector_count: u32,	//Number of sectors.
sector_size:  u32,	//Uniform sector size in bytes (0=sector_info used)
page_size:    u32,	////Optimal programming page size in bytes.
program_unit: u32,	//Smallest programmable unit in bytes.
erased_value: u32,	//Contents of erased memory (usually 0xFF)
reserved    : [u8; 3]	//Reserved (must be zero)
}


pub enum status_error_codes{
	ARM_DRIVER_OK(u32),
    ARM_DRIVER_ERROR,
    ARM_DRIVER_ERROR_BUSY,
    ARM_DRIVER_ERROR_TIMEOUT,
    ARM_DRIVER_ERROR_UNSUPPORTED,
    ARM_DRIVER_ERROR_PARAMETER,
    ARM_DRIVER_ERROR_SPECIFIC,

}

pub trait Flash<'a> {
	fn TM4C_Flash_Initialize(&mut self) -> status_error_codes;
	fn TM4C_Flash_Uninitialize(&mut self) -> status_error_codes;
	fn TM4C_Flash_ReadData(&mut self, addr: u32, data: [u32; MAX_READABLE_WORDS], num_items: u8) -> status_error_codes;
	fn TM4C_Flash_ProgramData(&mut self, addr: u32, data: [u32; MAX_WRITABLE_WORDS])-> status_error_codes;
	fn TM4C_Flash_EraseSector(&mut self, addr: u32) -> status_error_codes;
	fn TM4C_Flash_EraseChip(&mut self) -> status_error_codes;
	fn TM4C_Flash_GetStatus(&mut self);
	fn TM4C_Flash_GetInfo(&mut self);

}

pub struct tm4c_flash_unit{
	info: TM4C_FLASH_INFO,
}

//All the registers we need from tm4c123x crate

// #define FLASH_FMA_R             (*((volatile uint32_t *)0x400FD000))
// #define FLASH_FMA_OFFSET_MAX    0x0003FFFF  // Address Offset max
// #define FLASH_FMD_R             (*((volatile uint32_t *)0x400FD004))
// #define FLASH_FMC_R             (*((volatile uint32_t *)0x400FD008))
// #define FLASH_FMC_WRKEY         0xA4420000  // FLASH write key (KEY bit of FLASH_BOOTCFG_R set)
// #define FLASH_FMC_WRKEY2        0x71D50000  // FLASH write key (KEY bit of FLASH_BOOTCFG_R cleared)
// #define FLASH_FMC_MERASE        0x00000004  // Mass Erase Flash Memory
// #define FLASH_FMC_ERASE         0x00000002  // Erase a Page of Flash Memory
// #define FLASH_FMC_WRITE         0x00000001  // Write a Word into Flash Memory
// #define FLASH_FMC2_R            (*((volatile uint32_t *)0x400FD020))
// #define FLASH_FMC2_WRBUF        0x00000001  // Buffered Flash Memory Write
// #define FLASH_FWBN_R            (*((volatile uint32_t *)0x400FD100))
// #define FLASH_BOOTCFG_R         (*((volatile uint32_t *)0x400FE1D0))
// #define FLASH_BOOTCFG_KEY       0x00000010  // KEY Select
impl Flash <'_> for tm4c_flash_unit{

	fn TM4C_Flash_Initialize(&mut self) -> status_error_codes{

		//Nothing is really done here. THe only thing is to just make sure the power block and clocks are initialized

		status_error_codes::ARM_DRIVER_ERROR_SPECIFIC

	}
	fn TM4C_Flash_Uninitialize(&mut self) -> status_error_codes{

		status_error_codes::ARM_DRIVER_ERROR_SPECIFIC

	}
	fn TM4C_Flash_ReadData(&mut self, addr: u32, data: [u32; MAX_READABLE_WORDS], num_items: u8) -> status_error_codes{

		status_error_codes::ARM_DRIVER_ERROR_SPECIFIC

	}
	fn TM4C_Flash_ProgramData(&mut self, addr: u32, data: [u32; MAX_WRITABLE_WORDS])-> status_error_codes{
//   uint32_t flashkey;
  // uint32_t volatile *FLASH_FWBn_R = (uint32_t volatile*)0x400FD100;
  // int writes = 0;
  // if(MassWriteAddrValid(addr)){
  //   DisableInterrupts();                            // may be optional step
  //   while(FLASH_FMC2_R&FLASH_FMC2_WRBUF){           // wait for hardware idle
  //                // to do later: return ERROR if this takes too long
  //                // remember to re-enable interrupts
  //   };
  //   while((writes < 32) && (writes < count)){
  //     FLASH_FWBn_R[writes] = source[writes];
  //     writes = writes + 1;
  //   }
  //   FLASH_FMA_R = addr;
  //   if(FLASH_BOOTCFG_R&FLASH_BOOTCFG_KEY){          // by default, the key is 0xA442
  //     flashkey = FLASH_FMC_WRKEY;
  //   } else{                                         // otherwise, the key is 0x71D5
  //     flashkey = FLASH_FMC_WRKEY2;
  //   }
  //   FLASH_FMC2_R = (flashkey|FLASH_FMC2_WRBUF);     // start writing
  //   while(FLASH_FMC2_R&FLASH_FMC2_WRBUF){
  //                // to do later: return ERROR if this takes too long
  //                // remember to re-enable interrupts
  //   };           // wait for completion (~3 to 4 usec)
  //   EnableInterrupts();
  // }
		status_error_codes::ARM_DRIVER_ERROR_SPECIFIC

	}
	fn TM4C_Flash_EraseSector(&mut self, addr: u32) -> status_error_codes{
  // uint32_t flashkey;
  // if(EraseAddrValid(addr)){
  //   DisableInterrupts();                            // may be optional step
  //                                                   // wait for hardware idle
  //   while(FLASH_FMC_R&(FLASH_FMC_WRITE|FLASH_FMC_ERASE|FLASH_FMC_MERASE)){
  //                // to do later: return ERROR if this takes too long
  //                // remember to re-enable interrupts
  //   };
  //   FLASH_FMA_R = addr;
  //   if(FLASH_BOOTCFG_R&FLASH_BOOTCFG_KEY){          // by default, the key is 0xA442
  //     flashkey = FLASH_FMC_WRKEY;
  //   } else{                                         // otherwise, the key is 0x71D5
  //     flashkey = FLASH_FMC_WRKEY2;
  //   }
  //   FLASH_FMC_R = (flashkey|FLASH_FMC_ERASE);       // start erasing 1 KB block
  //   while(FLASH_FMC_R&FLASH_FMC_ERASE){
  //                // to do later: return ERROR if this takes too long
  //                // remember to re-enable interrupts
  //   };           // wait for completion (~3 to 4 usec)
  //   EnableInterrupts();
  //   return NOERROR;
  // }
  // return ERROR;

		status_error_codes::ARM_DRIVER_ERROR_SPECIFIC

	}
	fn TM4C_Flash_EraseChip(&mut self) -> status_error_codes{

		//call erase sector on all sectors

		status_error_codes::ARM_DRIVER_ERROR_SPECIFIC

	}
	fn TM4C_Flash_GetStatus(&mut self){

	}
	fn TM4C_Flash_GetInfo(&mut self){

	}

}
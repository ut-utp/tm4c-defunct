use lc3_isa::{
    Addr, Instruction, Word,
};
pub enum DiskError {
    DiskIsNotReal,
    AddressOutOfRange { capacity: Addr, given: Addr },
}

pub trait Disk: Default {
    fn is_real(&self) -> bool;
    fn capacity(&self) -> Addr;

    fn read(&self, addr: Addr) -> Result<Word, DiskError>;
    fn write(&mut self, addr: Addr, data: Word) -> Result<(), DiskError>;
}
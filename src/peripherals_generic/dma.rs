/// A singleton that represents a single DMA channel associated with a particular peripheral (would be a uart port for communicaion purposes)
///
/// This singleton has exclusive access to the registers of the peripheral associated dma chnnel registers
// Determine what trait bounds the Peripheral type should have
pub struct Dma1Channel <Peripheral> {
    peripheral: Peripheral
}


//A trait for  a dma channel. A physical peripheral
trait DmaChannel {

    //Device secific preinitialization to enable DMA
    fn dma_device_init(&mut self);

    /// Data will be written to this `address`
    ///
    /// `inc` indicates whether the address will be incremented after every byte transfer
    ///
    /// NOTE this performs a volatile write
    fn dma_set_destination_address(&mut self, address: usize, inc: bool);

    /// Data will be read from this `address`
    ///
    /// `inc` indicates whether the address will be incremented after every byte transfer
    ///
    /// NOTE this performs a volatile write
    fn dma_set_source_address(&mut self, address: usize, inc: bool);

    /// Number of bytes to transfer
    ///
    /// NOTE this performs a volatile write
    fn dma_set_transfer_length(&mut self, len: usize);

    /// Starts the DMA transfer
    ///
    /// NOTE this performs a volatile write
    fn dma_start(&mut self);

    /// Stops the DMA transfer
    ///
    /// NOTE this performs a volatile write
    fn dma_stop(&mut self);

    /// Returns `true` if there's a transfer in progress
    ///
    /// NOTE this performs a volatile read
    fn dma_in_progress() -> bool;
}



/// A singleton that represents a serial port
pub struct Serial {
    // ..
}

impl Serial {
    /// Reads out a single byte
    ///
    /// NOTE: blocks if no byte is available to be read
    pub fn read(&mut self) -> Result<u8, ()> {
        unimplemented!()
    }

    /// Sends out a single byte
    ///
    /// NOTE: blocks if the output FIFO buffer is full
    pub fn write(&mut self, byte: u8) -> Result<(), ()> {
        unimplemented!()
    }
}

impl DmaChannel for Serial {

    fn dma_device_init(&mut self){
         unimplemented!()
    }

    fn dma_set_destination_address(&mut self, address: usize, inc: bool) {
         unimplemented!()
        // ..
    }

    fn dma_set_source_address(&mut self, address: usize, inc: bool){
        unimplemented!()
    }

    fn dma_set_transfer_length(&mut self, len: usize){
        unimplemented!()
    }

    fn dma_start(&mut self){
        unimplemented!()
    }

    fn dma_stop(&mut self){
        unimplemented!()
    }

    fn dma_in_progress() -> bool{
        unimplemented!()
    }
}

/// A DMA transfer
pub struct Transfer<B> {
    buffer: B,
}

impl<B> Transfer<B> {
    /// Returns `true` if the DMA transfer has finished
    pub fn is_done(&self) -> bool {
        unimplemented!()
    }

    /// Blocks until the transfer is done and returns the buffer
    pub fn wait(self) -> B {
        // Busy wait until the transfer is done
        while !self.is_done() {}

        self.buffer
    }
}

impl Serial {
    /// Sends out the given `buffer`
    ///
    /// Returns a value that represents the in-progress DMA transfer
    pub fn dma_write_all<'a>(mut self, buffer: &'a [u8]) -> Transfer<&'a [u8]> {

        self.dma_device_init();
        // self.set_destination_address(UART_TX, false);
        // self.set_source_address(buffer.as_ptr() as usize, true);
        self.dma_set_transfer_length(buffer.len());

        self.dma_start();

        Transfer { buffer }
    }

    pub fn read_exact<'a>(&mut self, buffer: &'a mut [u8]) -> Transfer<&'a mut [u8]> {
        // self.set_source_address(UART_RX, false);
        // self.set_destination_address(buffer.as_mut_ptr() as usize, true);
        self.dma_set_transfer_length(buffer.len());

        self.dma_start();

        Transfer { buffer }
    }
}
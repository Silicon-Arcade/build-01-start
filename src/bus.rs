pub trait Bus {
    fn memory_read(&mut self, addr: u16) -> u8;
    fn memory_write(&mut self, addr: u16, value: u8);
}

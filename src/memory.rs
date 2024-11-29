struct Memory {
    memory: [u8; 65536],
}

trait MemoryOperations {
    fn fetch_byte(&self, address: usize) -> Option<u8>;
    fn store_byte(&mut self, address: usize, value: u8);
}

impl MemoryOperations for Memory {
    fn fetch_byte(&self, address: usize) -> Option<u8> {
        if address >= self.memory.len() {
            None
        }else {
            Some(self.memory[address])
        }
    }

    fn store_byte(&mut self, address: usize, value: u8) {
        if address >= self.memory.len() {}
    }
}
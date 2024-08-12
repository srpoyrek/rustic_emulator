pub struct Memory {
        data: Vec<u8>,
}



impl Memory {
    pub fn new() -> Self {
        Self {
            data: vec![0b00000000, 0b00000000, 0b01010000, 0b10110111, 
            0b00000000, 0b00000000, 0b01100001, 0b00110111,   
            0b00000000, 0b00100000, 0b10000001, 0b10110011,],
        }
    }
    
    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn load_word(&self, address: usize) -> u32 {
        // Ensure the address is within bounds and aligned to 4 bytes
        assert!(address % 4 == 0, "Unaligned memory access");
        assert!(address + 4 <= self.data.len(), "Address out of bounds");

        // Load the 4 bytes from memory
        let byte0: u32 = self.data[address] as u32;
        let byte1: u32 = self.data[address + 1] as u32;
        let byte2: u32 = self.data[address + 2] as u32;
        let byte3: u32 = self.data[address + 3] as u32;

        // Combine the bytes into a single 32-bit word (Little Endian)
        (byte0 << 24) | (byte1 << 16) | (byte2 << 8) | byte3
    }

    pub fn load_byte(&self, address: usize) -> u32 {
        // Ensure the address is within bounds
        assert!(address < self.data.len(), "Address out of bounds");
        // Load the a byte from memory
        self.data[address] as u32
    }


    pub fn load_halfword(&self, address: usize) -> u32 {
        // Ensure the address is within bounds and aligned to 2 bytes
        assert!(address % 2 == 0, "Unaligned memory access");
        assert!(address + 2 <= self.data.len(), "Address out of bounds");
    
        // Load the 2 bytes from memory
        let byte0: u32 = self.data[address] as u32;
        let byte1: u32 = self.data[address + 1] as u32;
    
        // Combine the bytes into a single 16-bit halfword (Little Endian)
        (byte0 << 8) | byte1
    }

    pub fn store_byte(&mut self, address: usize, value: u8) {
        // Ensure the address is within bounds
        assert!(address < self.data.len(), "Address out of bounds");

        // Store the byte at the given address
        self.data[address] = value;
    }

    pub fn store_halfword(&mut self, address: usize, value: u16) {
        // Ensure the address is within bounds and aligned to 2 bytes
        assert!(address % 2 == 0, "Unaligned memory access");
        assert!(address + 2 <= self.data.len(), "Address out of bounds");

        // Store the 2 bytes at the given address (Little Endian)
        self.data[address] = (value & 0xFF) as u8;        // Lower byte
        self.data[address + 1] = (value >> 8) as u8;      // Higher byte
    }

    pub fn store_word(&mut self, address: usize, value: u32) {
        // Ensure the address is within bounds and aligned to 4 bytes
        assert!(address % 4 == 0, "Unaligned memory access");
        assert!(address + 4 <= self.data.len(), "Address out of bounds");

        // Store the 4 bytes at the given address (Little Endian)
        self.data[address] = (value & 0xFF) as u8;          // Byte 0
        self.data[address + 1] = ((value >> 8) & 0xFF) as u8; // Byte 1
        self.data[address + 2] = ((value >> 16) & 0xFF) as u8; // Byte 2
        self.data[address + 3] = ((value >> 24) & 0xFF) as u8; // Byte 3
    }
    
}
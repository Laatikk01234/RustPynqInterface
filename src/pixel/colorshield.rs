pub struct ColorShield {
    pub memory_address: *mut u8,
    pub input: bool,
    pub clock: bool,
    pub bank: bool,
    pub latch: bool,
    pub reset: bool,
    pub active_column: usize,
    pub channel_address: *mut u8,
}

impl ColorShield {
    pub const fn new() -> ColorShield {
        ColorShield {
            memory_address: 0x41220008 as *mut u8,
            input: false,
            clock: false,
            bank: false,
            latch: false,
            reset: false,
            active_column: 0b00000000,
            channel_address: 0x41220000 as *mut u8,
        }
    }

    // return bit representation of active channel
    pub unsafe fn get_active_column_bit(&mut self) -> u8 {
        core::ptr::read_volatile(self.channel_address)
    }

    // returns integer representation of active channel
    pub unsafe fn get_active_column_int(&mut self) -> u8 {
        match core::ptr::read_volatile(self.channel_address) {
            0b1 => 1,
            0b10 => 2,
            0b100 => 3,
            0b1000 => 4,
            0b10000 => 5,
            0b100000 => 6,
            0b1000000 => 7,
            0b10000000 => 8,
            _ => 0,
        }
    }
    #[allow(dead_code)]
    pub unsafe fn set_active_column(&mut self, col_number: u8) {
        let translated = match col_number {
            1 => 0b1,
            2 => 0b10,
            3 => 0b100,
            4 => 0b1000,
            5 => 0b10000,
            6 => 0b100000,
            7 => 0b1000000,
            8 => 0b10000000,
            _ => 0b0,
        };
        self.active_column = translated;
        self.open_line(self.active_column as u8);
    }

    pub unsafe fn next_activating_column(&self) -> usize {
        let toreturn;
        // if no columns are active set first active
        if self.active_column == 0 {
            toreturn = 0b1;
        }
        // if current column is the last activate the first column
        else if self.active_column & 0b10000000 > 0 {
            toreturn = self.active_column >> 7;
        }
        // activate next column
        else {
            toreturn = self.active_column << 1;
        }

        match toreturn {
            0b1 => 1,
            0b10 => 2,
            0b100 => 3,
            0b1000 => 4,
            0b10000 => 5,
            0b100000 => 6,
            0b1000000 => 7,
            0b10000000 => 8,
            _ => 0,
        }
    }
    #[allow(dead_code)]
    pub unsafe fn set_next_column_active(&mut self) {
        // if no columns are active set first active
        if self.active_column == 0 {
            self.active_column = 0b1;
        }
        // if current column is the last activate the first column
        else if self.active_column & 0b10000000 > 0 {
            self.active_column >>= 7;
        }
        // activate next column
        else {
            self.active_column <<= 1;
        }
        self.open_line(self.active_column as u8);
    }

    /// Sets one line, matching with the parameter, as active.
    pub unsafe fn open_line(&mut self, i: u8) {
        core::ptr::write_volatile(self.channel_address, i);
    }

    pub unsafe fn set_input(&mut self, value: bool) {
        self.input = value;
        if self.input == true {
            mutate_ptr(self.memory_address, |x| x | 0b10000);
        } else {
            mutate_ptr(self.memory_address, |x| x & !0b10000);
        }
    }

    pub unsafe fn set_clock(&mut self, value: bool) {
        self.clock = value;
        if self.clock == true {
            mutate_ptr(self.memory_address, |x| x | 0b01000);
        } else {
            mutate_ptr(self.memory_address, |x| x & !0b01000);
        }
    }

    pub unsafe fn set_bank(&mut self, value: bool) {
        self.bank = value;
        if self.bank == true {
            mutate_ptr(self.memory_address, |x| x | 0b00100);
        } else {
            mutate_ptr(self.memory_address, |x| x & !0b00100);
        }
    }

    pub unsafe fn set_latch(&mut self, value: bool) {
        self.latch = value;
        if self.latch == true {
            mutate_ptr(self.memory_address, |x| x | 0b00010);
        } else {
            mutate_ptr(self.memory_address, |x| x & !0b00010);
        }
    }

    pub unsafe fn set_reset(&mut self, value: bool) {
        self.reset = value;
        if self.reset == true {
            mutate_ptr(self.memory_address, |x| x | 0b00001);
        } else {
            mutate_ptr(self.memory_address, |x| x & !0b00001);
        }
    }

    //reset
    pub unsafe fn reset(&mut self) {
        self.set_reset(false);
        self.set_reset(true);
    }

    //tick_clock
    pub unsafe fn tick_clock(&mut self) {
        self.set_clock(false);
        self.set_clock(true);
    }

    //latch
    pub unsafe fn latch(&mut self) {
        self.set_latch(true);
        self.set_latch(false);
    }

    //activate 6-bit bank
    pub unsafe fn activate_6_bit_bank(&mut self) {
        self.set_bank(false);
    }

    //activate 8-bit bank
    pub unsafe fn activate_8_bit_bank(&mut self) {
        self.set_bank(true);
    }
}

unsafe fn mutate_ptr<A, F>(addr: *mut A, mutate_fn: F)
where
    F: FnOnce(A) -> A,
{
    let prev = core::ptr::read_volatile(addr);
    let new = mutate_fn(prev);
    core::ptr::write_volatile(addr, new);
}

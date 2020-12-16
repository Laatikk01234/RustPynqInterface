use crate::{println64, print64};

use crate::xil;
//use crate::LED_ADDRESS;


//statics for clock
static mut IS_SETUP: bool = false;
static mut SECONDS: u8 = 0;
static mut MINUTES: u8 = 0;
static mut HOURS: u8 = 0;

/// Table for dots. Indices are page, x, y, color. Initialized to zero.
static mut DOTS: [[[u8; 3]; 8]; 8] = [[[0; 3]; 8]; 8];

// Memory address for colorshield functionality
const COLORSHIELD_ADDRESS: *mut u8 = 0x41220008 as *mut u8;

// ColorShield struct that is used to operate colorshield.
static mut COLOR_SHIELD: ColorShield = ColorShield::constructor();

// Memory address for active channels
const CHANNEL_ADDRESS: *mut u8 = 0x41220000 as *mut u8;

// variable that indicates active channel
static mut COLUMN: usize = 0b00000000;

// return bit representation of active channel
pub unsafe fn get_active_column_bit() -> u8 {
    core::ptr::read_volatile(CHANNEL_ADDRESS)
}

// returns integer representation of active channel
pub unsafe fn get_active_column_int() -> u8 {
    match COLUMN {
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

pub unsafe fn set_active_column(col_number: u8) {
    let translated: usize;
    match col_number {
        1 => {translated = 0b1},
        2 => {translated = 0b10},
        3 => {translated = 0b100},
        4 => {translated = 0b1000},
        5 => {translated = 0b10000},
        6 => {translated = 0b100000},
        7 => {translated = 0b1000000},
        8 => {translated = 0b10000000},
        _ => {translated = 0b0},
    }
    COLUMN = translated;
    run(COLUMN);
}

pub unsafe fn set_next_column_active() {
    // if no columns are active set first active
    if COLUMN == 0 {
        COLUMN = 0b1;
    }
    // if current column is the last activate the first column
    else if COLUMN & 0b10000000 > 0 {
        COLUMN >>= 7;   
    }
    // activate next column
    else {
        COLUMN <<= 1;
    }

    // run active column
    run(COLUMN);
}

struct ColorShield {
    memory_address: *mut u8,
    input: bool,
    clock: bool,
    bank: bool,
    latch: bool,
    reset: bool,
}

impl ColorShield {

    const fn constructor() -> ColorShield {
        ColorShield {
        memory_address: 0x41220008 as *mut u8,
        input: false,
        clock: false,
        bank: false,
        latch: false,
        reset: false }
    }

    unsafe fn print_status(&self) {
    println64!("Bits: {:05b}",core::ptr::read_volatile(self.memory_address));
    println64!("-> colorshields active channels are:");
    println64!("input = {}",self.input);
    println64!("clock = {}",self.clock);
    println64!("bank = {}" ,self.bank);
    println64!("latch = {}",self.latch);
    println64!("reset = {}",self.reset);
    println64!();
    }

    unsafe fn set_input(&mut self, value: bool) {
        self.input = value;
            if (self.input == true) {
                mutate_ptr(self.memory_address, |x| x | 0b10000);
            } else {
                mutate_ptr(self.memory_address, |x| x & !0b10000);
            }
    }

    unsafe fn set_clock(&mut self, value: bool) {
        self.clock = value;
            if (self.clock == true) {
                mutate_ptr(self.memory_address, |x| x | 0b01000);
            } else {
                mutate_ptr(self.memory_address, |x| x & !0b01000);
            }
    }

    unsafe fn set_bank(&mut self, value: bool) {
        self.bank = value;
            if (self.bank == true) {
                mutate_ptr(self.memory_address, |x| x | 0b00100);
            } else {
                mutate_ptr(self.memory_address, |x| x & !0b00100);
            }
    }

    unsafe fn set_latch(&mut self, value: bool) {
        self.latch = value;
            if (self.latch == true) {
                mutate_ptr(self.memory_address, |x| x | 0b00010);
            } else {
                mutate_ptr(self.memory_address, |x| x & !0b00010);
            }
    }

    unsafe fn set_reset(&mut self, value: bool) {
        self.reset = value;
            if (self.reset == true) {
                mutate_ptr(self.memory_address, |x| x | 0b00001);
            } else {
                mutate_ptr(self.memory_address, |x| x & !0b00001);
            }
    }

    //reset
    unsafe fn reset(&mut self) {
        unsafe {
            self.set_reset(false);
            self.set_reset(true);
        }
    }

    //tick_clock
    unsafe fn tick_clock(&mut self) {
        unsafe {
            self.set_clock(false);
            self.set_clock(true);
        }
    }

    //latch
    unsafe fn latch(&mut self) {
        unsafe {
            self.set_latch(true);
            self.set_latch(false);
        }
    }

    //activate 6-bit bank
    unsafe fn activate_6_bit_bank(&mut self) {
        unsafe {
            self.set_bank(false);
        }
    }

    //activate 8-bit bank
    unsafe fn activate_8_bit_bank(&mut self) {
        unsafe {
            self.set_bank(true);
        }
    }
}


pub unsafe fn color_shield_status() {
    COLOR_SHIELD.print_status();
}

pub unsafe fn set_input(value: bool) {
    COLOR_SHIELD.set_input(value);
}

pub unsafe fn set_clock(value: bool) {
    COLOR_SHIELD.set_clock(value);
}

pub unsafe fn set_bank(value: bool) {
    COLOR_SHIELD.set_bank(value);
}

pub unsafe fn set_latch(value: bool) {
    COLOR_SHIELD.set_latch(value);
}

pub unsafe fn set_reset(value: bool) {
    COLOR_SHIELD.set_reset(value);
}

//reset
pub unsafe fn reset() {
    COLOR_SHIELD.reset();
}

//tick_clock
pub unsafe fn tick_clock() {
    COLOR_SHIELD.tick_clock();
}

//latch
pub unsafe fn latch() {
    COLOR_SHIELD.latch();
}

//activate 6-bit bank
pub unsafe fn activate_6_bit_bank() {
    COLOR_SHIELD.activate_6_bit_bank();
}

//activate 8-bit bank
pub unsafe fn activate_8_bit_bank() {
    COLOR_SHIELD.activate_8_bit_bank();
}

pub unsafe fn setup_led_matrix() {
    reset();
    activate_6_bit_bank();

    for _i in 0..24 {
        let brightness = 2;
        for x in 0..6 {
            if x > brightness {
                set_input(true);
            } else {
                set_input(false);
            }
            tick_clock();
        }
    }
    latch();

}

/// Set the value of one pixel at the LED matrix. Function is unsafe because it
/// uses global memory
/// coordinates in range 1-8 and colors of 0,255.
unsafe fn set_pixel(x: usize, y: usize, r: u8, g: u8, b: u8) {
    // Set new pixel value. Take the parameeters and put them into the
    // DOTS array.

    if (x >= 1 && x <= 8)
        && (y >= 1 && y <= 8) {
        DOTS[x-1][y-1][0] = r;
        DOTS[x-1][y-1][1] = g;
        DOTS[x-1][y-1][2] = b;

    } else {
        println64!("Bad coordinate values: X: {}, Y: {}",x,y);
    }

}

pub unsafe fn run(c: usize) {

    activate_8_bit_bank();


    let row: usize;
    match c {
        0b1 => {row = 0},
        0b10 => {row = 1},
        0b100 => {row = 2},
        0b1000 => {row = 3},
        0b10000 => {row = 4},
        0b100000 => {row = 5},
        0b1000000 => {row = 6},
        0b10000000 => {row = 7},
        _ => {row = 0},
    }

    //lights off for the duration of pushing new values to show
    open_line(0);

	for column in (0..8).rev()  { 
		for rgb in (0..3).rev() {
			if DOTS[row][column][rgb] >= 1 {
                set_input(true);
			} else {
                set_input(false);
			}
			for _ in 0..8 {
                tick_clock();
			}
		}
	}
    
	latch();
    

    open_line(c as u8);



    // TODO: Write into the LED matrix driver (8-bit data). Use values from DOTS
    // array.
}

/// Sets one line, matching with the parameter, as active.
pub unsafe fn open_line(i: u8) {
    core::ptr::write_volatile(CHANNEL_ADDRESS, i);
}

pub unsafe fn setup_clock(hours: u8, minutes: u8, seconds: u8) {
    HOURS = hours;
    MINUTES = minutes;
    SECONDS = seconds;
    clear_dots();
    IS_SETUP = true;
}

pub unsafe fn run_clock() {
    if IS_SETUP {
    increment_time();
    update_led_table();
    } else {
        setup_clock(0,0,0);
    }
}

unsafe fn increment_time() {
    SECONDS += 1;
    if SECONDS >= 60 {
        SECONDS = 0;
        MINUTES += 1;
        if MINUTES >= 60 {
            MINUTES = 0;
            HOURS += 1;
            if HOURS >= 24 {
                HOURS = 0;
            }
        }
    }
}

unsafe fn update_led_table() {
    //seconds
    let mut seconds: u8 = SECONDS.clone();
    // least significant numbers of seconds
    let mut seconds1: u8 = seconds % 10;
    // how many most significant number of seconds there are
    seconds = (seconds - seconds1) / 10;

    let mut minutes: u8 = MINUTES.clone();
    let mut minutes1: u8 = minutes % 10;
    minutes = (minutes - minutes1) / 10;

    let mut hours: u8 = HOURS.clone();
    let mut hours1: u8 = hours % 10;
    hours = (hours - hours1) / 10;


    //seconds
    for x in 1..5 {
        if seconds1 & 0b1 > 0 {
            set_pixel(8,x as usize,0,0,1);
        } else {
            set_pixel(8,x as usize,0,0,0);
        }
        seconds1 >>= 1;
    }

    for x in 1..4 {
        if seconds & 0b1 > 0 {
            set_pixel(7,x as usize,0,0,1);
        } else {
            set_pixel(7,x as usize,0,0,0);
        }
        seconds >>= 1;
    }

    //minutes
    for x in 1..5 {
        if minutes1 & 0b1 > 0 {
            set_pixel(5,x as usize,0,1,0);
        } else {
            set_pixel(5,x as usize,0,0,0);
        }
        minutes1 >>= 1;
    }

    for x in 1..4 {
        if minutes & 0b1 > 0 {
            set_pixel(4,x as usize,0,1,0);
        } else {
            set_pixel(4,x as usize,0,0,0);
        }
        minutes >>= 1;
    }

    //hours
    for x in 1..5 {
        if hours1 & 0b1 > 0 {
            set_pixel(2,x as usize,1,0,0);
        } else {
            set_pixel(2,x as usize,0,0,0);
        }
        hours1 >>= 1;
    }

    for x in 1..4 {
        if hours & 0b1 > 0 {
            set_pixel(1,x as usize,1,0,0);
        } else {
            set_pixel(1,x as usize,0,0,0);
        }
        hours >>= 1;
    }
}

pub unsafe fn clear_dots() {
    for x in 0..8 {
        for y in 0..8 {
            for z in 0..3 {
                DOTS[x][y][z] = 0;
            }
        }
    }
}

/// A helper one-liner for mutating raw pointers at given address. You shouldn't need to change this.
unsafe fn mutate_ptr<A, F>(addr: *mut A, mutate_fn: F)
where
    F: FnOnce(A) -> A,
{
    let prev = core::ptr::read_volatile(addr);
    let new = mutate_fn(prev);
    core::ptr::write_volatile(addr, new);
}

use crate::{print64, println64};

use crate::xil;
//use crate::LED_ADDRESS;

//use colorshield;
mod colorshield;

//statics for clock
static mut IS_SETUP: bool = false;
static mut SECONDS: u8 = 0;
static mut MINUTES: u8 = 0;
static mut HOURS: u8 = 0;

/// Table for dots. Indices are page, x, y, color. Initialized to zero.
static mut DOTS: [[[u8; 3]; 8]; 8] = [[[0; 3]; 8]; 8];

// ColorShield struct that is used to operate colorshield.
static mut COLOR_SHIELD: colorshield::ColorShield = colorshield::ColorShield::new();

// return bit representation of active channel
#[allow(dead_code)]
pub unsafe fn get_active_column_bit() -> u8 {
    COLOR_SHIELD.get_active_column_bit()
}

// returns integer representation of active channel
#[allow(dead_code)]
pub unsafe fn get_active_column_int() -> u8 {
    COLOR_SHIELD.get_active_column_int()
}

#[allow(dead_code)]
pub unsafe fn set_active_column(col_number: u8) {
    COLOR_SHIELD.set_active_column(col_number);
}

#[allow(dead_code)]
pub unsafe fn set_next_column_active() {
    run(COLOR_SHIELD.next_activating_column());
}

pub fn color_shield_status() {
    println64!("Bits: {:05b}", unsafe {
        core::ptr::read_volatile(COLOR_SHIELD.memory_address)
    });
    println64!("-> colorshields active channels are:");
    println64!("input = {}", unsafe { COLOR_SHIELD.input });
    println64!("clock = {}", unsafe { COLOR_SHIELD.clock });
    println64!("bank = {}", unsafe { COLOR_SHIELD.bank });
    println64!("latch = {}", unsafe { COLOR_SHIELD.latch });
    println64!("reset = {}", unsafe { COLOR_SHIELD.reset });
    println64!();
}

#[allow(dead_code)]
pub unsafe fn set_input(value: bool) {
    COLOR_SHIELD.set_input(value);
}

#[allow(dead_code)]
pub unsafe fn set_clock(value: bool) {
    COLOR_SHIELD.set_clock(value);
}

#[allow(dead_code)]
pub unsafe fn set_bank(value: bool) {
    COLOR_SHIELD.set_bank(value);
}

#[allow(dead_code)]
pub unsafe fn set_latch(value: bool) {
    COLOR_SHIELD.set_latch(value);
}

#[allow(dead_code)]
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
pub fn set_pixel(x: usize, y: usize, r: u8, g: u8, b: u8) {
    // Set new pixel value. Take the parameeters and put them into the
    // DOTS array.

    if (x >= 1 && x <= 8) && (y >= 1 && y <= 8) {
        unsafe {
            DOTS[x - 1][y - 1][0] = r;
            DOTS[x - 1][y - 1][1] = g;
            DOTS[x - 1][y - 1][2] = b;
        }
    } else {
        println64!("Bad coordinate values: X: {}, Y: {}", x, y);
    }
}

pub unsafe fn run(c: usize) {
    activate_8_bit_bank();
    let column: usize = c - 1;

    //lights off for the duration of pushing new values to show
    set_active_column(0);
    for row in (0..8).rev() {
        for rgb in (0..3).rev() {
            if DOTS[column][row][rgb] >= 1 {
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
    set_active_column(c as u8);
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
        setup_clock(0, 0, 0);
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
            set_pixel(8, x as usize, 0, 0, 1);
        } else {
            set_pixel(8, x as usize, 0, 0, 0);
        }
        seconds1 >>= 1;
    }

    for x in 1..4 {
        if seconds & 0b1 > 0 {
            set_pixel(7, x as usize, 0, 0, 1);
        } else {
            set_pixel(7, x as usize, 0, 0, 0);
        }
        seconds >>= 1;
    }

    //minutes
    for x in 1..5 {
        if minutes1 & 0b1 > 0 {
            set_pixel(5, x as usize, 0, 1, 0);
        } else {
            set_pixel(5, x as usize, 0, 0, 0);
        }
        minutes1 >>= 1;
    }

    for x in 1..4 {
        if minutes & 0b1 > 0 {
            set_pixel(4, x as usize, 0, 1, 0);
        } else {
            set_pixel(4, x as usize, 0, 0, 0);
        }
        minutes >>= 1;
    }

    //hours
    for x in 1..5 {
        if hours1 & 0b1 > 0 {
            set_pixel(2, x as usize, 1, 0, 0);
        } else {
            set_pixel(2, x as usize, 0, 0, 0);
        }
        hours1 >>= 1;
    }

    for x in 1..4 {
        if hours & 0b1 > 0 {
            set_pixel(1, x as usize, 1, 0, 0);
        } else {
            set_pixel(1, x as usize, 0, 0, 0);
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

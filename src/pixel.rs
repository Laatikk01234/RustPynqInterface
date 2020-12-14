use crate::{println64, print64};

use crate::xil;
use crate::LED_ADDRESS;


/// Table for dots. Indices are page, x, y, color. Initialized to zero.
static mut DOTS: [[[u8; 3]; 8]; 8] = [[[0; 3]; 8]; 8];

//pub const LED_ADDRESS: *mut u8 = 0x00000000 as *mut u8;
//#define colcmd *(uint8_t *) 0x41220008
pub const COLORSHIELD_ADDRESS: *mut u8 = 0x41220008 as *mut u8;

pub const CHANNEL_ADDRESS: *mut u8 = 0x41220000 as *mut u8;

static mut SECONDS: u8 = 0;
static mut MINUTES: u8 = 0;
static mut HOURS: u8 = 0;


const CLK_CTRL: *mut u8 = 0x00000000 as *mut u8;


pub unsafe fn setup_led_matrix() {
    // Tip: use the following to set an ADDRESS to zero:
    /*
    core::ptr::write_volatile(ADDRESS, 0);
    */

    // The screen must be reset at start
    // Tip: use the following one-liners to flip bits on or off at ADDRESS. Oh
    // yes, it's a zero-cost lambda function in an embedded application.
    /*
    mutate_ptr(ADDR, |x| x | 1);
    mutate_ptr(ADDR, |x| x ^ 1);
    */

    // TODO: Write code that sets 6-bit values in register of DM163 chip. It is
    // recommended that every bit in that register is set to 1. 6-bits and 24
    // "bytes", so some kind of loop structure could be nice

    core::ptr::write_volatile(COLORSHIELD_ADDRESS, 0);
    //reset board (twice?)
    mutate_ptr(COLORSHIELD_ADDRESS, |x| x & 0b00000);
    //set input 0, clock 0, shift bank low, latch low, reset off
    mutate_ptr(COLORSHIELD_ADDRESS, |x| x | 0b00001);

    for i in 0..24 {
        let mut brightness: u8 = 0b000111;
        
        for x_ in 0..6 {
            //set brightness accoring to its value
            if (brightness & 0b100000) > 0 { 
                //input 1 tai 0 riippuen brightness arvosta
                mutate_ptr(COLORSHIELD_ADDRESS, |x| x | 0b10000);
            } else {
                mutate_ptr(COLORSHIELD_ADDRESS, |x| x & !0b10000);
            }

            //kello alas
            mutate_ptr(COLORSHIELD_ADDRESS,|x| x & !0b01000);
            //kello ylos
            mutate_ptr(COLORSHIELD_ADDRESS,|x| x | 0b01000);
            brightness<<=1;
        
        }
        
    }
    latch();
    

    
    set_pixel(1,3,0,1,0);
    set_pixel(8,8,1,0,0);
    set_pixel(3,4,0,0,1);
    set_pixel(3,1,0,255,0);
    

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

/// Refresh new data into the LED matrix. Hint: This function is supposed to
/// send 24-bytes and parameter x is for x-coordinate.
pub unsafe fn run(c: usize) {

    core::ptr::write_volatile(COLORSHIELD_ADDRESS,0b00101);
    //mutate_ptr(COLORSHIELD_ADDRESS, |x| x | ?)


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
			if (DOTS[row][column][rgb] >= 1) {
                mutate_ptr(COLORSHIELD_ADDRESS, |x| x | 0b10000);
			} else {
                mutate_ptr(COLORSHIELD_ADDRESS, |x| x & !0b10000);
			}
			for _ in 0..8 {
                mutate_ptr(COLORSHIELD_ADDRESS, |x| x & !0b01000);
                mutate_ptr(COLORSHIELD_ADDRESS, |x| x | 0b01000);
			}
		}
	}
    
	latch();
    

    open_line(c as u8);



    // TODO: Write into the LED matrix driver (8-bit data). Use values from DOTS
    // array.
}

/// Latch signal for the colors shield. See colorsshield.pdf for how latching
/// works.
unsafe fn latch() {
    //latch locks bits at rising edge -> set latch on and off.
    mutate_ptr(COLORSHIELD_ADDRESS, |x| x | 0b00010);
    mutate_ptr(COLORSHIELD_ADDRESS, |x| x & !0b00010);
}

/// Sets one line, matching with the parameter, as active.
pub unsafe fn open_line(i: u8) {
    core::ptr::write_volatile(CHANNEL_ADDRESS, i);

    // TODO: Write code here.
    // Tip: use a `match` statement for the parameter:
    /*
    match i {
        0 => {},
        _ => {},
    }
    */
}

pub unsafe fn setup_clock(hours: u8, minutes: u8, seconds: u8) {
    HOURS = hours;
    MINUTES = minutes;
    SECONDS = seconds;
    clear_dots();
}

pub unsafe fn run_clock() {
    increment_time();
    update_led_table();
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
    // 
    println64!("{}",seconds1);
    println64!("{}",seconds);

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

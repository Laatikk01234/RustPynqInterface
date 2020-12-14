use crate::{println64, print64};

use crate::xil;
use crate::LED_ADDRESS;

const PAGE_SIZE: usize = 10;

pub static mut PAGE: usize = 0;

/// Table for dots. Indices are page, x, y, color. Initialized to zero.
//static mut DOTS: [[[[u8; PAGE_SIZE]; 8]; 8]; 3] = [[[[0; PAGE_SIZE]; 8]; 8]; 3];
static mut DOTS: [[[u8; 3]; 8]; 8] = [[[0; 3]; 8]; 8];

//pub const LED_ADDRESS: *mut u8 = 0x00000000 as *mut u8;
//#define colcmd *(uint8_t *) 0x41220008
pub const COLORSHIELD_ADDRESS: *mut u8 = 0x41220008 as *mut u8;

pub const CHANNEL_ADDRESS: *mut u8 = 0x41220000 as *mut u8;


const CLK_CTRL: *mut u8 = 0x00000000 as *mut u8;


pub unsafe fn setup_led_matrix() {

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

    set_pixel(2,3,1,0,0);
    set_pixel(4,6,1,0,0);
    set_pixel(7,7,0,0,1);
    set_pixel(1,7,0,0,1);
    set_pixel(0,0,0,1,0);
    set_pixel(0,3,0,1,0);
    set_pixel(3,0,0,1,0);
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
}

/// Set the value of one pixel at the LED matrix. Function is unsafe because it
/// uses global memory
unsafe fn set_pixel(x: usize, y: usize, r: u8, g: u8, b: u8) {
    DOTS[x][y][0] = r;
    DOTS[x][y][1] = g;
    DOTS[x][y][2] = b;
    // TODO: Set new pixel value. Take the parameeters and put them into the
    // DOTS array.
}

/// Refresh new data into the LED matrix. Hint: This function is supposed to
/// send 24-bytes and parameter x is for x-coordinate.
pub unsafe fn run(c: usize) {

    core::ptr::write_volatile(COLORSHIELD_ADDRESS,0b00101);
    //mutate_ptr(COLORSHIELD_ADDRESS, |x| x | ?)

    //valot pois
    open_line(0);

    let row: usize;
    match c {
        2 => {row = 1},
        4 => {row = 2},
        8 => {row = 3},
        16 => {row = 4},
        32 => {row = 5},
        64 => {row = 6},
        128 => {row = 7},
        _ => {row = 0},
    }
    //println64!("{}",DOTS[2][3][0]);
    
	for column in 0..8  {
    //println64!("{}",column);   
		for rgb in (0..3).rev() {
        //println64!("{}",rgb);
			if (DOTS[row][column][rgb] == 1) {
                mutate_ptr(COLORSHIELD_ADDRESS, |x| x | 0b10000);
				//colcmd |= 0b10000;
			} else {
                mutate_ptr(COLORSHIELD_ADDRESS, |x| x & !0b10000);
				//colcmd &= ~0b10000;
			}
			for _ in 0..8 {
                mutate_ptr(COLORSHIELD_ADDRESS, |x| x & !0b01000);
                mutate_ptr(COLORSHIELD_ADDRESS, |x| x | 0b01000);
				//colcmd &= ~0b01000;
				//colcmd |= 0b01000;
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

/// A helper one-liner for mutating raw pointers at given address. You shouldn't need to change this.
unsafe fn mutate_ptr<A, F>(addr: *mut A, mutate_fn: F)
where
    F: FnOnce(A) -> A,
{
    let prev = core::ptr::read_volatile(addr);
    let new = mutate_fn(prev);
    core::ptr::write_volatile(addr, new);
}

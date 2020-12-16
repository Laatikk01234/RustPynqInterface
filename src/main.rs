//! Implements the led-blinker course work on Xilinx Pynq-Z1 SoC.

// The `no_std` attribute prevents Rust standard library from being built into the binary. This is
// necessary, because the standard library is not available for baremetal Cortex-A9. The Rust `std`
// is in many ways similar to the C++ `std`.
#![no_std]
// Open feature gates to some particular extras related to low-level hacking that are not part of
// the core Rust language: assembler, language keyword overwriting and program entry point.
#![feature(asm, lang_items, start)]

mod interrupt;
mod pixel;
mod print;

// xil_sys contains the Xilinx Cortex-A9 board support package (BSP) and a Rust
// FFI. We rename the module here as `xil`.
use xil_sys as xil;

// Re-import symbols from pixel without the `pixel::` prefix
use pixel::*;

// Rust `core` imports for using C-style void-pointers and info for a custom
// panic implementation.
use core::{ffi::c_void, panic::PanicInfo};

// Declare static globals like in the C-version. This is a reasonable way of
// communicating between threads in interrupt-driven concurrency.

pub static mut COLUMN: usize = 1;
pub static mut HIDASTIN: usize = 1;

// Define the address of the ordinary LED interface in physical memory. Putting
// bits into the LED address the right way may cause desired blinking of
// hardware LEDs.
// FIXME: 0x00000000 is not the LED address. The correct address can be found in
// some of the provided documentation.
pub const LED_ADDRESS: *mut u8 = 0x00000000 as *mut u8;



// The #[start] attribute is usually not necessary, but we need to show the
// cross-compiler where to start executing. The underscore before the argument
// signals that the parameter is not used.
#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    // Initialize board interrupt functions
    // N.B. Do not touch this function, concurrency is set up here
    interrupt::init();

    // An unsafe block for setting up the LED-matrix using the C-API, and for
    // touching a static global.
    unsafe {
        color_shield_status();
        setup_led_matrix();
        color_shield_status();
        println64!("{}",get_active_column_bit());
        println64!("{}",get_active_column_int());
        set_active_column(3);
        println64!("Active column(s): {:08b}",get_active_column_bit());
        println64!("Active column: {}",get_active_column_int());
        
        //println64!("Active column: {}",);
        // Setting a static global requires an `unsafe` block in Rust, because the
        // compiler cannot verify soundness in a case where an interrupt causes
        // simultaneous access from another thread. Thus we must make sure ourselves,
        // not to do that.
    }

    unsafe {
        // Enables the board to break execution of the main thread using an interrupt
        // request (IRQ), and jump onto the interrupt handler. Direct calls to C API
        // functions (`xil::*`) are `unsafe` by default, because the compiler
        // does not verify soundness of C code.
        xil::Xil_ExceptionEnable();
    }

    // Prints up to 64 characters using standard Rust [print formatting](https://doc.rust-lang.org/std/fmt/index.html).
    println64!("Hello Rust!");

    
    //println64!("{}",0b10000 & 0b01000);

    // Empty loop to keep the program running while the interrupt handlers do all the
    // work
    loop {}
}

/// Interrupt handler for switch and buttons. This function gets called on
/// switch and button interrupts. Connected buttons are at bank 2.
///
/// # Arguments
///
/// * `status` - a binding containing one flipped bit to match the source of the
///   interrupt. See line comments of contained `match` statement.
pub unsafe extern "C" fn button_handler(callback_ref: *mut c_void, _bank: u32, status: u32) {
    // Don't mind me, line is for brevity
    // N.B. Removing this line is totally okay
    let _gpio = callback_ref as *mut xil::XGpioPs;

    // TODO: Write code here
    // Tip: use a match-statement to pattern-match button status. The match
    // statement takes the `status` parameter binding and matches it to
    // different binary patterns (eg. 0b001 for decimal 1, or 0b100 for
    // decimal 4). You can use binary, decimal or hex for the match, but I
    // found the binary representation more readable.
    /*
    match status {
        // No buttons are pressed
        0b000000 => {},
        // TODO: match into a pattern here
        // ??? => ???
        // `_` is the 'rest' pattern, that is handled if no other variant matches above
        _ => {},
    }
    */
    // End of your code
}

/// Timer interrupt handler for led matrix update. The function updates only one
/// line (`CHANNEL`) of the matrix per call, but sets `channel` as the next line
/// to be updated. `pub extern "C"` qualifier is required to allow passing the
/// handler to the C API.
pub unsafe extern "C" fn tick_handler(callback_ref: *mut c_void) {
    // Exceptions need to be disabled during screen update.
    xil::Xil_ExceptionDisable();

    // TODO: Write code here
    /*
        run(COLUMN);
     
        if COLUMN & 0b10000000 > 0 {
            COLUMN >>= 7;
        
        } else {
            COLUMN <<= 1;
        }
    */
    set_next_column_active();
 
    // End of your code

    // Cast `void*` received from the C API to the "Triple Timer Counter" (TTC)
    // instance pointer. The C API needs to use void pointers to pass data around,
    // because the C specification does not describe abstract data types (ADT).
    let ttc = callback_ref as *mut xil::XTtcPs;

    // Clear timer interrupt status
    // N.B. Do not remove
    let status_event = xil::XTtcPs_GetInterruptStatus(ttc);
    xil::XTtcPs_ClearInterruptStatus(ttc, status_event);

    // Put exceptions back on (previously disabled at the start of the interrupt
    // handler)
    xil::Xil_ExceptionEnable();
}

/// Timer interrupt handler for moving the alien, shooting, and other game
/// logic. See also [tick_handler](fn.tick_handler.html) and its line comments
/// for details.
pub unsafe extern "C" fn tick_handler_1(callback_ref: *mut c_void) {
    // TODO: Write code here
    if HIDASTIN >= 10 {
        //run_clock();
        HIDASTIN = 0;
    } else {
        HIDASTIN += 1;
    }
        
    // End of your code

    // Clear timer interrupt status
    // N.B. Do not remove
    let ttc = callback_ref as *mut xil::XTtcPs;
    let status_event = xil::XTtcPs_GetInterruptStatus(ttc);
    xil::XTtcPs_ClearInterruptStatus(ttc, status_event);
}

/// A custom panic handler for Cortex-A9
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // logs "panicked at '$reason', src/main.rs:27:4" to host stdout
    println64!("{}", info);

    loop {}
}

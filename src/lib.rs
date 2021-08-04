#![no_std]
#![allow(dead_code)]
#![feature(core_intrinsics, core_panic, lang_items, asm, repr_simd)]

// use core::intrinsics::abort;
use core::panic::PanicInfo;

pub mod pi;


#[no_mangle]
pub extern fn kernel_main() {
    // write("Hello Rust Kernel world!  ");
    pi::act_led::set_as_output();
    // pi::act_led::turn_on();
    pi::act_led::flash(1);
    pi::act_led::sleep(0xFF0000);
    loop {
        let t = pi::timer::get_clochi();
        pi::act_led::blink_bin_64(t);
        pi::act_led::sleep(0xFF0000);
        pi::act_led::sleep(0xFF0000);
    }
}


// These functions below provide definitions for symbols libcore
// expects which are not present on our bare metal target. You
// will not encounter linker errors until you use a part of
// libcore that references them, such as iterators in this program.
// In the future you may need to provide real implementations for
// these functions.

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() {}

#[lang = "eh_personality"]
pub extern fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        pi::act_led::blink_morse_str("sos");
        pi::act_led::sleep(0xFF0000);
        pi::act_led::sleep(0xFF0000);
    }
    // unsafe { abort() }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn _Unwind_Resume() { loop {} }
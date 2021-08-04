// Some for the sleep() call
const RAPID: u32 = 0x01fe00; //  SHORT / 16
const RAPID2: u32 = 0x03fc00; //  SHORT / 8
const QUICK: u32 = 0x07f800; // SHORT / 4
const QUICK2: u32 = 0x0ff000; // SHORT / 2
const SHORT: u32 = 0x1fe000;
const SHORT2: u32 = 0x3fc000; // = SHORT * 2
const LONG: u32 = 0x7f8000;   // = SHORT2 * 2
const LONG2: u32 = 0xFF0000;  // = LONG * 2

pub fn set_as_output() {
    unsafe {
        asm!("ldr r0,=0x20200010");
        asm!("ldr r1,=0x1");
        asm!("lsl r1,#21");
        asm!("str r1,[r0]");
    }
}

pub fn turn_on() {
    unsafe {
        asm!("ldr r0,=0x2020002C");
        asm!("ldr r1,=0x1");
        asm!("lsl r1,#15");
        asm!("str r1,[r0]");
    }
}

pub fn turn_off() {
    unsafe {
        asm!("ldr r0,=0x20200020");
        asm!("ldr r1,=0x1");
        asm!("lsl r1,#15");
        asm!("str r1,[r0]");
    }
}

pub fn blink_led(hex_countdown: u32) {
    unsafe {
        turn_on();
        sleep(hex_countdown);
        turn_off();
        sleep(hex_countdown);
    }
}

pub fn blink_loop() {
    unsafe {
        asm!("1:");
        turn_on();
        sleep(SHORT);
        turn_off();
        sleep(SHORT);
        asm!("b 1b");
    }
}

pub fn blink_loop_slow() {
    unsafe {
        asm!("2:");
        turn_on();
        sleep(LONG);
        turn_off();
        sleep(LONG);
        asm!("b 2b");
    }
}

pub fn sleep(hex_countdown: u32) {
    /*
    To create a delay, we busy the processor on a pointless quest to
    decrement the countdown to 0. Which takes around a second for 0xFF000.
    We fist substract 1 from register 2.
    We later compare r2 with 0, which result is placed in an special register.
    We branch if the special register holds false.
    */
    unsafe {
        asm!("mov r2,{0}", in(reg) hex_countdown);
        asm!("wait1$:");
        asm!("sub r2,#1");
        asm!("cmp r2,#0");
        asm!("bne wait1$");
    }
}

pub fn flash(times: u8) {
    for i in 0..times {
        turn_on();
        sleep(RAPID2);
        turn_off();
        sleep(QUICK);
    }
}

fn blink_bin_digit(n: bool) {
    match n {
        false => {
            turn_on();
            sleep(QUICK2);
            turn_off();
            sleep(SHORT2);
        },
        true => {
            turn_on();
            sleep(SHORT2);
            turn_off();
            sleep(SHORT2);
        }
    }
}

/// Blink the given number in binary (lo to hi)
pub fn blink_bin(n: u32) {
    flash(3);
    sleep(SHORT);
    blink_bin_digit(((n >> 0) & 1) != 0);
    blink_bin_digit(((n >> 1) & 1) != 0);
    blink_bin_digit(((n >> 2) & 1) != 0);
    blink_bin_digit(((n >> 3) & 1) != 0);
    blink_bin_digit(((n >> 4) & 1) != 0);
    blink_bin_digit(((n >> 5) & 1) != 0);
    blink_bin_digit(((n >> 6) & 1) != 0);
    blink_bin_digit(((n >> 7) & 1) != 0);
    blink_bin_digit(((n >> 8) & 1) != 0);
    blink_bin_digit(((n >> 9) & 1) != 0);
    blink_bin_digit(((n >> 10) & 1) != 0);
    blink_bin_digit(((n >> 11) & 1) != 0);
    blink_bin_digit(((n >> 12) & 1) != 0);
    blink_bin_digit(((n >> 13) & 1) != 0);
    blink_bin_digit(((n >> 14) & 1) != 0);
    blink_bin_digit(((n >> 15) & 1) != 0);

    blink_bin_digit(((n >> 16) & 1) != 0);
    blink_bin_digit(((n >> 17) & 1) != 0);
    blink_bin_digit(((n >> 18) & 1) != 0);
    blink_bin_digit(((n >> 19) & 1) != 0);
    blink_bin_digit(((n >> 20) & 1) != 0);
    blink_bin_digit(((n >> 21) & 1) != 0);
    blink_bin_digit(((n >> 22) & 1) != 0);
    blink_bin_digit(((n >> 23) & 1) != 0);
    blink_bin_digit(((n >> 24) & 1) != 0);
    blink_bin_digit(((n >> 25) & 1) != 0);
    blink_bin_digit(((n >> 26) & 1) != 0);
    blink_bin_digit(((n >> 27) & 1) != 0);
    blink_bin_digit(((n >> 28) & 1) != 0);
    blink_bin_digit(((n >> 29) & 1) != 0);
    blink_bin_digit(((n >> 30) & 1) != 0);
    blink_bin_digit(((n >> 31) & 1) != 0);
}

/// Blink the given number in binary (lo to hi)
pub fn blink_bin_64(n: u64) {
    let n1: u32 = n as u32;
    let n2: u32 = (n >> 32) as u32;
    blink_bin(n1);
    // blink_bin(n2);

    // I just need these first four digits of CHI for now
    blink_bin_digit(((n2 >> 0) & 1) != 0);
    blink_bin_digit(((n2 >> 1) & 1) != 0);
    blink_bin_digit(((n2 >> 2) & 1) != 0);
    blink_bin_digit(((n2 >> 3) & 1) != 0);
}

pub fn blink_morse(c: char) {
    match c {
        'a' => {
            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

        },
        'b' => {
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

        },
        'c' => {
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

        },
        'd' => {
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

        },
        'e' => {
            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

        },
        'f' => {
            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

        },
        'g' => {
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

        },
        'h' => {
            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

        },
        'i' => {
            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

        },
        'j' => {
            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

        },
        'k' => {
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

        },
        'l' => {
            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

        },
        'm' => {
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

        },
        'n' => {
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

        },
        'o' => {
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

        },
        'p' => {
            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

        },
        'q' => {
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

        },
        'r' => {
            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

        },
        's' => {
            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

        },
        't' => {
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

        },
        'u' => {
            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

        },
        'v' => {
            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

        },
        'w' => {
            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

        },
        'x' => {
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

        },
        'y' => {
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

        },
        'z' => {
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

        },
        '0' => {
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

        },
        '1' => {
            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

        },
        '2' => {
            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

        },
        '3' => {
            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

        },
        '4' => {
            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

        },
        '5' => {
            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

        },
        '6' => {
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

        },
        '7' => {
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

        },
        '8' => {
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

        },
        '9' => {
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);
            // -
            turn_on();
            sleep(LONG);
            turn_off();
            sleep(SHORT);

            // .
            turn_on();
            sleep(SHORT);
            turn_off();
            sleep(SHORT);

        },
        ' ' => {
            turn_off();
            sleep(LONG2);
        }
        _ => {}
    }
}

pub fn blink_morse_str(s: &str) {
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        blink_morse(c);
    }
}

pub fn blink_morse_int(i: u8) {
    match i {
        0 => blink_morse('0'),
        1 => blink_morse('1'),
        2 => blink_morse('2'),
        3 => blink_morse('3'),
        4 => blink_morse('4'),
        5 => blink_morse('5'),
        6 => blink_morse('6'),
        7 => blink_morse('7'),
        8 => blink_morse('8'),
        9 => blink_morse('9'),
        _ => {
            // todo
            blink_morse('x');
        }
    }
}

pub fn sos() {
    blink_morse_str("sos");
}
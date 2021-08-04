
use core::intrinsics::volatile_load;
// use core::intrinsics::volatile_store;

/// Addresses based on Broadcom peripherals docs, BCM2835-ARM-Peripherals.pdf
/// translated from bus address space (starting at 0x7E000000)
/// to physical address space (starting at 0x20000000)

const PERIPHERAL_BASE: u32 = 0x20000000; //
const SYS_TIMER_OFFSET: u32 = 0x00003000; // page 172
const ARM_TIMER_OFFSET: u32 = 0x0000B000; // page 196

// Broadcom Peripherals (BCM2835-ARM-Peripherals.pdf) page 172
// pub enum SysTimer {
//     CS  = SYS_TIMER_BASE,
//     CLO = SYS_TIMER_BASE + 0x4,
//     CHI = SYS_TIMER_BASE + 0x8,
//     C0  = SYS_TIMER_BASE + 0xc,
//     C1  = SYS_TIMER_BASE + 0x10,
//     C2  = SYS_TIMER_BASE + 0x14,
//     C3  = SYS_TIMER_BASE + 0x18
// }

pub fn get_clo() -> u32 {
    mmio_read(PERIPHERAL_BASE + SYS_TIMER_OFFSET + 0x4)
}

pub fn get_chi() -> u32 {
    mmio_read(PERIPHERAL_BASE + SYS_TIMER_OFFSET + 0x8)
}

pub fn get_clochi() -> u64 {
    mmio_read(PERIPHERAL_BASE + SYS_TIMER_OFFSET + 0x4).into()
}


fn mmio_read(reg: u32) -> u32 {
    unsafe { volatile_load(reg as *const u32) }
}
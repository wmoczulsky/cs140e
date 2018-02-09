#![feature(compiler_builtins_lib, lang_items, asm, pointer_methods)]
#![no_builtins]
#![no_std]

extern crate compiler_builtins;

pub mod lang_items;

const GPIO_BASE: usize = 0x3F000000 + 0x200000;

const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;

const RAND_CONST: *mut u32 = 0x20000000 as *mut u32;

#[inline(never)]
fn spin_sleep_ms(ms: usize) {
    for _ in 0..(ms * 6000) {
        unsafe { RAND_CONST.write_volatile(RAND_CONST.read_volatile() + 1) }
        // unsafe { asm!("nop" :::: "volatile"); }
    }
}

fn set_bit(ptr: *mut u32, bit_n: u32) {
    unsafe {
        ptr.write_volatile(1u32 << bit_n);
    }
}

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    set_bit(GPIO_FSEL1, 18);
    loop {
        set_bit(GPIO_CLR0, 16);
        spin_sleep_ms(100);
        set_bit(GPIO_SET0, 16);
        spin_sleep_ms(100);
    }
}

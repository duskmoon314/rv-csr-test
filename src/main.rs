#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(asm)]
#![feature(panic_info_message)]

// use console::ANSICON;
#[macro_use]
extern crate log;
use riscv::register::{sideleg, sie, sip, uie, uip};
#[allow(unused_imports)]
use riscv::register::{sstatus, ustatus};


#[macro_use]
mod console;
mod lang_items;
mod logger;
mod sbi;
mod stack;
mod trap;

global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
pub fn rust_main() -> ! {
    logger::init();
    clear_bss();
    trap::init();
    info!("Hello rv-csr-test");
    info!("{:#x?}", ustatus::read());
    unsafe {
        sstatus::set_sie();
        sie::set_ssoft();
        sie::set_usoft();
        sip::set_ssoft();
        sip::set_usoft();
    }

    // extern "C" {
    //     fn foo();
    // }

    unsafe {
        sstatus::clear_sie();
        sideleg::set_usoft();
    }

    let sp: usize = stack::USER_STACK.get_sp();
    let entry: usize;
    let mut s: [usize; 12] = [0; 12];
    unsafe {
        asm!("la {}, foo", out(reg) entry);
        asm!("mv {}, s0", out(reg) s[0]);
        asm!("mv {}, s1", out(reg) s[1]);
        asm!("mv {}, s2", out(reg) s[2]);
        asm!("mv {}, s3", out(reg) s[3]);
        asm!("mv {}, s4", out(reg) s[4]);
        asm!("mv {}, s5", out(reg) s[5]);
        asm!("mv {}, s6", out(reg) s[6]);
        asm!("mv {}, s7", out(reg) s[7]);
        asm!("mv {}, s8", out(reg) s[8]);
        asm!("mv {}, s9", out(reg) s[9]);
        asm!("mv {}, s10", out(reg) s[10]);
        asm!("mv {}, s11", out(reg) s[11]);
    }

    let ctx = stack::KERNEL_STACK.push_ucontext(trap::UserTrapContext::init(entry, sp, s));

    extern "C" {
        fn __restore_u(cx_addr: usize);
    }

    unsafe {
        __restore_u(ctx as *const _ as usize);
    }

    unsafe {
        asm!(
            "
    .global foo
    foo:
    nop
    "
        );
    }

    trap::init_u();

    unsafe {
        asm!("nop");
    }

    unsafe {
        uie::set_usoft();
        uip::set_usoft();
    }

    info!("user mode");

    panic!("Shutdown machine!");
}

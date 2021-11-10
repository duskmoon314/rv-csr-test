#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(asm)]
#![feature(panic_info_message)]

// use console::ANSICON;
#[macro_use]
extern crate log;
use crate::{
    config::{CLOCK_FREQ, CPU_NUM},
    sbi::{send_ipi, set_timer},
    user_uart::{get_base_addr_from_irq, PollingSerial},
};
use core::sync::atomic::{AtomicBool, Ordering::Relaxed};
use embedded_hal::{prelude::_embedded_hal_serial_Write, serial::Read};
use riscv::register::{sideleg, sip, sstatus, time, uie, uip, ustatus};
#[cfg(feature = "board_lrv")]
use uart_xilinx::uart_lite::MmioUartAxiLite;

#[macro_use]
mod console;
mod config;
mod lang_items;
mod logger;
mod plic;
mod sbi;
mod stack;
mod trap;
mod user_uart;

static IS_TIMEOUT: AtomicBool = AtomicBool::new(false);
static IS_HART1_INIT: AtomicBool = AtomicBool::new(false);
pub const BAUD_RATE: usize = 6_250_000;

global_asm!(include_str!("entry.asm"));

pub fn hart_id() -> usize {
    let hart_id: usize;
    unsafe {
        asm!("mv {}, tp", out(reg) hart_id);
    }
    hart_id
}

fn clear_bss() {
    extern "C" {
        fn s_bss();
        fn e_bss();
        fn e_bss_ma();
    }
    println!(
        "s_bss: {:#x?}, e_bss: {:#x?}, e_bss_ma: {:#x?}",
        s_bss as usize, e_bss as usize, e_bss_ma as usize
    );
    (s_bss as usize..e_bss_ma as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
pub fn rust_main(hart_id: usize) -> ! {
    trap::init();
    if hart_id == 0 {
        clear_bss();
        println!("Hello rv-csr-test");
        logger::init();
        println!("logger init finished");
        info!("{:#x?}", ustatus::read());
        for i in 1..CPU_NUM {
            debug!("Start {}", i);
            let mask: usize = 1 << i;
            send_ipi(&mask as *const _ as usize);
        }
        while !IS_HART1_INIT.load(Relaxed) {}
    } else {
        info!("Hart booted");
        IS_HART1_INIT.store(true, Relaxed);
    }

    uart_speed_test_multihart(hart_id);
    delay(1000);
    unsafe {
        sip::set_ssoft();
        sip::set_usoft();
    }

    uart_speed_test();
    // extern "C" {
    //     fn foo();
    // }

    unsafe {
        sstatus::clear_sie();
        sideleg::set_usoft();
        asm!("csrr zero, sideleg");
        asm!("csrr zero, sedeleg");
    }

    let sp: usize = stack::USER_STACK.get_sp();
    let entry: usize;
    let mut s: [usize; 12] = [0; 12];
    unsafe {
        asm!("la {}, 2f", out(reg) entry);
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
    2:
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

#[cfg(feature = "board_lrv")]
fn uart_lite_test() {
    plic::init();
    let uart = MmioUartAxiLite::new(0x6000_0000);
    uart.enable_interrupt();
    for i in 0..64 {
        uart.write_byte(i as u8 + 'A' as u8);
    }
    info!("uart0 status: {:#x?}", uart.status());
    for _ in 0..1000_000 {}
    info!("uart0 status: {:#x?}", uart.status());
    plic::handle_external_interrupt();
}

fn uart_speed_test() {
    #[cfg(feature = "board_qemu")]
    let mut uart1 = PollingSerial::new(get_base_addr_from_irq(14));
    #[cfg(feature = "board_qemu")]
    let mut uart2 = PollingSerial::new(get_base_addr_from_irq(15));

    #[cfg(feature = "board_lrv")]
    let mut uart1 = PollingSerial::new(get_base_addr_from_irq(6));
    #[cfg(feature = "board_lrv")]
    let mut uart2 = PollingSerial::new(get_base_addr_from_irq(7));

    uart1.hardware_init(BAUD_RATE);
    uart2.hardware_init(BAUD_RATE);
    let t = time::read();
    set_timer(t + CLOCK_FREQ);
    while !IS_TIMEOUT.load(Relaxed) {
        for _ in 0..14 {
            let _ = uart1.try_write(0x55);
            let _ = uart2.try_write(0x55);
        }
        for _ in 0..14 {
            let _ = uart1.try_read();
            let _ = uart2.try_read();
        }
    }

    info!("uart1 rx {}, tx {}", uart1.rx_count, uart1.tx_count);
    info!("uart2 rx {}, tx {}", uart2.rx_count, uart2.tx_count);
}

fn delay(ms: usize) {
    let start = time::read();
    while time::read() - start < CLOCK_FREQ * ms / 1000 {}
}

fn uart_speed_test_multihart(hart_id: usize) {
    #[cfg(feature = "board_qemu")]
    let mut uart1 = PollingSerial::new(get_base_addr_from_irq(14 + hart_id as u16));

    #[cfg(feature = "board_lrv")]
    let mut uart1 = PollingSerial::new(get_base_addr_from_irq(6 + hart_id as u16));

    uart1.hardware_init(BAUD_RATE);
    let t = time::read();
    set_timer(t + CLOCK_FREQ);
    while !IS_TIMEOUT.load(Relaxed) {
        for _ in 0..14 {
            let _ = uart1.try_write(0x55);
        }
        for _ in 0..14 {
            let _ = uart1.try_read();
        }
    }
    if hart_id == 1 {
        delay(100);
    }
    info!("uart rx {}, tx {}", uart1.rx_count, uart1.tx_count);
}

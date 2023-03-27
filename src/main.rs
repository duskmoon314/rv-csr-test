#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(naked_functions)]
#![feature(asm_const)]

// use console::ANSICON;
#[macro_use]
extern crate log;
extern crate alloc;
use core::arch::{asm, global_asm};

use crate::{
    config::{CLOCK_FREQ, CPU_NUM},
    plic::Plic,
    sbi::{send_ipi, set_timer},
    user_uart::{get_base_addr_from_irq, BufferedSerial, PollingSerial},
};
use blake3::Hasher;
use core::sync::atomic::{AtomicBool, Ordering::Relaxed};
use embedded_hal::{prelude::_embedded_hal_serial_Write, serial::Read};
use lazy_static::*;
use riscv::register::{sideleg, sie, sip, sstatus, time, uie, uip, ustatus};
#[cfg(feature = "board_lrv")]
use uart_xilinx::uart_lite::MmioUartAxiLite;

#[macro_use]
mod console;
mod config;
mod lang_items;
mod logger;
mod mm;
mod plic;
mod sbi;
mod stack;
mod trap;
mod user_uart;

static IS_TIMEOUT: AtomicBool = AtomicBool::new(false);
static IS_HART1_INIT: AtomicBool = AtomicBool::new(false);
lazy_static! {
    pub static ref HAS_INTR: [AtomicBool; CPU_NUM] =
        array_init::array_init(|_| AtomicBool::new(false));
}
pub const BAUD_RATE: usize = 6_250_000;

/// Boot kernel size allocated in `_start` for single CPU.
pub const BOOT_STACK_SIZE: usize = 0x4_0000;

/// Total boot kernel size.
pub const TOTAL_BOOT_STACK_SIZE: usize = BOOT_STACK_SIZE * CPU_NUM;

// global_asm!(include_str!("entry.asm"));

/// Initialize kernel stack in .bss section.
#[link_section = ".bss.stack"]
static mut STACK: [u8; TOTAL_BOOT_STACK_SIZE] = [0u8; TOTAL_BOOT_STACK_SIZE];

/// Entry for the first kernel.
#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn __entry(hartid: usize) -> ! {
    core::arch::asm!(
        // Use tp to save hartid
        // "ebreak",
        "mv tp, a0",
        // Set stack pointer to the kernel stack.
        "
        la a1, {stack}
        li t0, {total_stack_size}
        li t1, {stack_size}
        mul sp, a0, t1
        sub sp, t0, sp
        add sp, a1, sp
        ",        // Jump to the main function.
        "j  {main}",
        total_stack_size = const TOTAL_BOOT_STACK_SIZE,
        stack_size       = const BOOT_STACK_SIZE,
        stack            =   sym STACK,
        main             =   sym rust_main_init,
        options(noreturn),
    )
}

/// Entry for other kernels.
#[naked]
#[no_mangle]
pub unsafe extern "C" fn __entry_others(hartid: usize) -> ! {
    core::arch::asm!(
        // Use tp to save hartid
        "mv tp, a0",
        // Set stack pointer to the kernel stack.
        "
        la a1, {stack}
        li t0, {total_stack_size}
        li t1, {stack_size}
        mul sp, a0, t1
        sub sp, t0, sp
        add sp, a1, sp
        ",
        // Jump to the main function.
        "j  {main}",
        total_stack_size = const TOTAL_BOOT_STACK_SIZE,
        stack_size       = const BOOT_STACK_SIZE,
        stack            =   sym STACK,
        main             =   sym rust_main_init_other,
        options(noreturn),
    )
}

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
    (s_bss as usize..e_bss_ma as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
    println!(
        "s_bss: {:#x?}, e_bss: {:#x?}, e_bss_ma: {:#x?}",
        s_bss as usize, e_bss as usize, e_bss_ma as usize
    );
}

#[no_mangle]
pub fn rust_main_init(hart_id: usize) {
    trap::init();
    clear_bss();
    println!("Hello rv-csr-test");
    logger::init();
    println!("logger init finished");
    plic::init();
    info!("{:#x?}", ustatus::read());
    // unsafe { asm!("csrwi 0x800, 1") }
    mm::init_heap();
    if CPU_NUM > 1 {
        for i in 0..CPU_NUM {
            if i != hart_id {
                debug!("Start {}", i);
                // let mask: usize = 1 << i;
                // send_ipi(&mask as *const _ as usize);

                // Starts other harts.
                let ret = sbi_rt::hart_start(i, __entry_others as _, 0);
                assert!(ret.is_ok(), "Failed to shart hart {}", i);
            }
        }
        while !IS_HART1_INIT.load(Relaxed) {}
    }
    rust_main(hart_id)
}

#[no_mangle]
pub fn rust_main_init_other(hart_id: usize) {
    trap::init();
    info!("Hart {} booted", hart_id);
    IS_HART1_INIT.store(true, Relaxed);
    rust_main(hart_id)
}

#[no_mangle]
pub fn rust_main(hart_id: usize) -> ! {
    info!("Tests begin!");
    #[cfg(feature = "board_lrv")]
    uart_lite_test_multihart_intr(hart_id, 'S');
    // unsafe { asm!("csrwi 0x800, 1") }
    // uart_speed_test_multihart(hart_id);
    // info!("polling mode test finished");
    // delay(1000);
    // uart_speed_test_multihart_intr(hart_id, 'S');
    // info!("interrupt mode test finished");
    // delay(1000);
    unsafe {
        sip::set_ssoft();
        sip::set_usoft();
    }

    // uart_speed_test();
    // extern "C" {
    //     fn foo();
    // }

    unsafe {
        sstatus::clear_sie();
        sstatus::set_spp(sstatus::SPP::User);
        sideleg::set_usoft();
        sideleg::set_uext();
        asm!("csrr zero, sideleg");
        asm!("csrr zero, sedeleg");
    }

    let sp: usize = stack::USER_STACK[hart_id].get_sp();
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

    let ctx = stack::KERNEL_STACK[hart_id].push_ucontext(trap::UserTrapContext::init(entry, sp, s));

    extern "C" {
        fn __restore_u_from_s(cx_addr: usize);
    }

    unsafe {
        __restore_u_from_s(ctx as *const _ as usize);
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
    uart_speed_test_multihart_intr(hart_id, 'U');
    #[cfg(feature = "board_lrv")]
    uart_lite_test_multihart_intr(hart_id, 'U');
    info!("test fin, waiting to shutdown...");
    delay(5000);
    panic!("Shutdown machine!");
}

#[cfg(feature = "board_lrv")]
#[allow(unused)]
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
    plic::handle_external_interrupt(0, 'S');
}

#[cfg(feature = "board_lrv")]
#[allow(unused)]
fn uart_lite_test_multihart_intr(hart_id: usize, mode: char) {
    plic::init_hart(hart_id);
    let context = plic::get_context(hart_id, mode);
    let irq = 4 + hart_id as u16;
    let uart = MmioUartAxiLite::new(get_base_addr_from_irq(irq));
    Plic::enable(context, irq);
    Plic::claim(context);
    Plic::complete(context, irq);
    // unsafe { asm!("csrwi 0x800, 1") }
    uart.enable_interrupt();
    match mode {
        'S' => unsafe {
            sie::set_sext();
        },
        'U' => unsafe {
            uie::set_uext();
        },
        _ => {
            error!("{} mode not supported!", mode);
        }
    }
    for i in 0..16 {
        uart.write_byte(i as u8 + 'A' as u8);
    }
    info!("uart{} status: {:#x?}", hart_id, uart.status());
    for _ in 0..1000_000 {}
    info!("uart{} status: {:#x?}", hart_id, uart.status());
    for _ in 0..1000_000 {}
    match mode {
        'S' => info!("sip: {:#x?}", sip::read()),
        'U' => info!("uip: {:#x?}", uip::read()),
        _ => {
            error!("{} mode not supported!", mode);
        }
    }
    uart.disable_interrupt();
    Plic::claim(context);
    Plic::complete(context, irq);
    Plic::disable(context, irq);

    // plic::handle_external_interrupt(hart_id, mode);
}

#[allow(unused)]
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

#[allow(unused)]
fn uart_speed_test_multihart(hart_id: usize) {
    info!("uart_speed_test_multihart");
    #[cfg(feature = "board_qemu")]
    let mut uart1 = PollingSerial::new(get_base_addr_from_irq(14 + hart_id as u16));

    #[cfg(feature = "board_lrv")]
    let mut uart1 = PollingSerial::new(get_base_addr_from_irq(6 + hart_id as u16));

    uart1.hardware_init(BAUD_RATE);
    let mut hasher = Hasher::new();
    IS_TIMEOUT.store(false, Relaxed);
    let t = time::read();
    set_timer(t + CLOCK_FREQ);

    let tx: u8 = 0;
    while !IS_TIMEOUT.load(Relaxed) {
        for _ in 0..14 {
            if let Ok(()) = uart1.try_write(tx) {
                hasher.update(&[tx]);
                tx.wrapping_add(1);
            }
        }
        for _ in 0..14 {
            if let Ok(ch) = uart1.try_read() {
                hasher.update(&[ch]);
            }
        }
    }
    if hart_id == 1 {
        delay(100);
    }
    info!("uart rx {}, tx {}", uart1.rx_count, uart1.tx_count);
}

#[allow(unused)]
fn uart_speed_test_multihart_intr(hart_id: usize, mode: char) {
    info!("uart_speed_test_multihart_intr");
    plic::init_hart(hart_id);
    let context = plic::get_context(hart_id, mode);

    #[cfg(feature = "board_qemu")]
    let irq = 14 + hart_id as u16;

    #[cfg(feature = "board_lrv")]
    let irq = 6 + hart_id as u16;

    let mut uart1 = BufferedSerial::new(get_base_addr_from_irq(irq));
    uart1.hardware_init(BAUD_RATE);
    let mut hasher = Hasher::new();
    Plic::enable(context, irq);
    IS_TIMEOUT.store(false, Relaxed);
    let t = time::read();
    set_timer(t + CLOCK_FREQ);
    match mode {
        'S' => unsafe {
            sie::set_sext();
        },
        'U' => unsafe {
            uie::set_uext();
        },
        _ => {
            error!("{} mode not supported!", mode);
        }
    }
    while !IS_TIMEOUT.load(Relaxed) {
        for _ in 0..14 {
            if let Ok(()) = uart1.try_write(0x55) {
                hasher.update(&[0x55]);
            }
        }
        for _ in 0..14 {
            if let Ok(ch) = uart1.try_read() {
                hasher.update(&[ch]);
            }
        }
        if HAS_INTR[hart_id].load(Relaxed) {
            uart1.interrupt_handler();
            HAS_INTR[hart_id].store(false, Relaxed);
            Plic::complete(context, irq);
            // info!("new intr!");
        }
    }
    delay(100);
    info!(
        "intr uart rx {}, tx {}, rx_intr {}, tx_intr {}",
        uart1.rx_count, uart1.tx_count, uart1.rx_intr_count, uart1.tx_intr_count
    );
}

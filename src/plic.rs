use core::sync::atomic::Ordering::Relaxed;
use rv_plic::Priority;
use rv_plic::PLIC;

use crate::HAS_INTR;

#[cfg(any(feature = "board_qemu", feature = "board_lrv"))]
pub const PLIC_BASE: usize = 0xc00_0000;
#[cfg(any(feature = "board_qemu", feature = "board_lrv"))]
pub const PLIC_PRIORITY_BIT: usize = 3;

pub type Plic = PLIC<PLIC_BASE, PLIC_PRIORITY_BIT>;

pub fn get_context(hartid: usize, mode: char) -> usize {
    const MODE_PER_HART: usize = 3;
    hartid * MODE_PER_HART
        + match mode {
            'M' => 0,
            'S' => 1,
            'U' => 2,
            _ => panic!("Wrong Mode"),
        }
}

#[cfg(feature = "board_qemu")]
pub fn init() {
    Plic::set_priority(14, Priority::lowest());
    Plic::set_priority(15, Priority::lowest());
}

#[cfg(feature = "board_lrv")]
pub fn init() {
    Plic::set_priority(6, Priority::lowest());
    Plic::set_priority(7, Priority::lowest());
}

#[cfg(feature = "board_qemu")]
pub fn init_hart(hart_id: usize) {
    let context = get_context(hart_id, 'S');
    Plic::enable(context, 14 + hart_id as u16);
    Plic::set_threshold(context, Priority::any());
}

#[cfg(feature = "board_lrv")]
pub fn init_hart(hart_id: usize) {
    let context = get_context(hart_id, 'S');
    Plic::clear_enable(context, 0);
    Plic::clear_enable(get_context(hart_id, 'U'), 0);
    Plic::enable(context, 6 + hart_id as u16);
    Plic::set_threshold(context, Priority::any());
    Plic::set_threshold(get_context(hart_id, 'M'), Priority::never());
}

pub fn handle_external_interrupt(hart_id: usize, mode: char) {
    let context = get_context(hart_id, mode);
    while let Some(_irq) = Plic::claim(context) {
        // debug!("[PLIC] IRQ: {:?}", irq);
        HAS_INTR[hart_id].store(true, Relaxed);
        // Plic::complete(context, irq)
    }
}

use rv_plic::Priority;
use rv_plic::PLIC;

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

pub fn handle_external_interrupt() {
    if let Some(irq) = Plic::claim(get_context(0, 'S')) {
        debug!("[PLIC] IRQ: {:?}", irq);
        match irq {
            #[cfg(feature = "board_qemu")]
            10 => {
                debug!("[PLIC] kenel handling uart");
            }
            #[cfg(feature = "board_lrv")]
            3 => {
                debug!("[PLIC] kenel handling uart");
            }
            _ => {
                warn!("[PLIC]: Not handle yet");
            }
        }

        Plic::complete(get_context(0, 'S'), irq)
    } else {
        warn!("[PLIC] No pending IRQ!");
    }
}

pub fn init() {
    Plic::set_threshold(1, Priority::any());
    Plic::set_threshold(2, Priority::any());
    #[cfg(feature = "board_qemu")]
    {
        Plic::enable(1, 10);
        Plic::set_priority(9, Priority::lowest());
        Plic::set_priority(10, Priority::lowest());
    }
    #[cfg(feature = "board_lrv")]
    {
        Plic::enable(1, 1);
        Plic::enable(1, 2);
        Plic::enable(1, 3);
        Plic::enable(1, 4);
        Plic::enable(1, 5);
        Plic::set_priority(1, Priority::lowest());
        Plic::set_priority(2, Priority::lowest());
        Plic::set_priority(3, Priority::lowest());
        Plic::set_priority(4, Priority::lowest());
        Plic::set_priority(5, Priority::lowest());
    }
}

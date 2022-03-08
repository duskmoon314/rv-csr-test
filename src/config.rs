pub const KERNEL_HEAP_SIZE: usize = 0x1_0000;
pub const USER_STACK_SIZE: usize = 0x400;
pub const KERNEL_STACK_SIZE: usize = 0x400;

#[cfg(feature = "board_qemu")]
pub const CLOCK_FREQ: usize = 12_500_000;

#[cfg(feature = "board_lrv")]
pub const CLOCK_FREQ: usize = 10_000_000;

pub const CPU_NUM: usize = 1;

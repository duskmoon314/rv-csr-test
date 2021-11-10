pub const USER_STACK_SIZE: usize = 0x4000;
pub const KERNEL_STACK_SIZE: usize = 0x4000;

#[cfg(feature = "board_qemu")]
pub const CLOCK_FREQ: usize = 12_500_000;

#[cfg(feature = "board_lrv")]
pub const CLOCK_FREQ: usize = 10_000_000;

pub const CPU_NUM: usize = 2;

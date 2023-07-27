use crate::axidma::AxiDmaConfig;

pub const KERNEL_HEAP_SIZE: usize = 0x10_0000;
pub const USER_STACK_SIZE: usize = 0x4000;
pub const KERNEL_STACK_SIZE: usize = 0x4000;

#[cfg(feature = "board_qemu")]
pub const CLOCK_FREQ: usize = 12_500_000;

#[cfg(feature = "board_lrv")]
pub const CLOCK_FREQ: usize = 10_000_000;

pub const CPU_NUM: usize = 1;

pub const TRIGGER_GPIO_BASE: usize = 0x6020_0000;
pub const NET_DEVICE_BASE: usize = 0x6011_0000;

pub const AXI_DMA_CONFIG: AxiDmaConfig = AxiDmaConfig {
    device_id: 0,
    base_address: 0x6010_0000,
    has_sts_cntrl_strm: false,
    is_micro_dma: false,
    has_mm2s: true,
    has_mm2s_dre: true,
    mm2s_data_width: 64,
    mm2s_burst_size: 64,
    has_s2mm: true,
    has_s2mm_dre: true,
    s2mm_data_width: 64,
    s2mm_burst_size: 8,
    has_sg: true,
    sg_length_width: 16,
    addr_width: 32,
};

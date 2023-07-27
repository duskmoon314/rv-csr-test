mod bd;
mod bd_ring;
mod hw;

use axidma_pac;

use alloc::{sync::Arc, vec::Vec};
use core::{
    arch::asm,
    sync::atomic::{
        compiler_fence, fence, AtomicUsize,
        Ordering::{Relaxed, SeqCst},
    },
};
use core::{ops::Deref, pin::Pin};
use lazy_static::lazy_static;
use spin::Mutex;

use crate::config::AXI_DMA_CONFIG;

use self::{
    bd_ring::{AxiDmaBdRing, AxiDmaBdRingConfig},
    hw::{AXIDMA_RX_OFFSET, AXIDMA_TX_OFFSET},
};

pub static TX_FRAMES: AtomicUsize = AtomicUsize::new(0);
pub static RX_FRAMES: AtomicUsize = AtomicUsize::new(0);

pub struct AxiDma {
    base_address: usize,
    has_sg: bool,
    is_micro_dma: bool,
    is_initialized: bool,

    tx_bd_ring: Option<AxiDmaBdRing>,
    rx_bd_ring: Option<AxiDmaBdRing>,
    addr_width: isize,
}

pub struct AxiDmaConfig {
    pub device_id: u32,
    pub base_address: usize,

    pub has_sts_cntrl_strm: bool,
    pub is_micro_dma: bool,

    pub has_mm2s: bool,
    pub has_mm2s_dre: bool,
    pub mm2s_data_width: usize,
    pub mm2s_burst_size: usize,

    pub has_s2mm: bool,
    pub has_s2mm_dre: bool,
    pub s2mm_data_width: usize,
    pub s2mm_burst_size: usize,

    pub has_sg: bool,
    pub sg_length_width: usize,
    pub addr_width: isize,
}

pub struct AxiDmaIntr {
    base_address: usize,
}

impl Default for AxiDma {
    fn default() -> Self {
        AxiDma::new(AXI_DMA_CONFIG)
    }
}

lazy_static! {
    pub static ref AXI_DMA: Arc<Mutex<AxiDma>> = Arc::new(Mutex::new(AxiDma::default()));
    pub static ref AXI_DMA_INTR: Arc<Mutex<AxiDmaIntr>> =
        Arc::new(Mutex::new(AxiDmaIntr::new(AXI_DMA_CONFIG.base_address)));
}

impl AxiDma {
    const RESET_TIMEOUT: isize = 500;
    pub fn new(config: AxiDmaConfig) -> Self {
        let max_transfer_len = (1usize << config.sg_length_width) - 1;
        let tx_bd_ring = if config.has_mm2s {
            Some(AxiDmaBdRing::new(AxiDmaBdRingConfig {
                chan_base_addr: config.base_address + AXIDMA_TX_OFFSET,
                is_rx_chan: false,
                has_sts_cntrl_strm: config.has_sts_cntrl_strm,
                has_dre: config.has_mm2s_dre,
                data_width: (config.mm2s_data_width >> 3),
                addr_ext: (config.addr_width > 32),
                max_transfer_len: if config.is_micro_dma {
                    config.mm2s_data_width / 8 * config.mm2s_burst_size
                } else {
                    max_transfer_len
                },
            }))
        } else {
            None
        };

        let rx_bd_ring = if config.has_s2mm {
            Some(AxiDmaBdRing::new(AxiDmaBdRingConfig {
                chan_base_addr: config.base_address + AXIDMA_RX_OFFSET,
                is_rx_chan: true,
                has_sts_cntrl_strm: config.has_sts_cntrl_strm,
                has_dre: config.has_s2mm_dre,
                data_width: (config.s2mm_data_width >> 3),
                addr_ext: (config.addr_width > 32),
                max_transfer_len: if config.is_micro_dma {
                    config.s2mm_data_width / 8 * config.s2mm_burst_size
                } else {
                    max_transfer_len
                },
            }))
        } else {
            None
        };

        let mut ret = Self {
            base_address: config.base_address,
            has_sg: config.has_sg,
            is_micro_dma: config.is_micro_dma,
            addr_width: config.addr_width,
            tx_bd_ring,
            rx_bd_ring,
            is_initialized: false,
        };
        ret.reset();
        let mut timeout = AxiDma::RESET_TIMEOUT;
        while timeout > 0 && !ret.reset_is_done() {
            timeout -= 1;
        }
        if timeout > 0 {
            ret.is_initialized = true;
        } else {
            error!("AXIDMA: failed reset in intialization");
        }
        ret
    }

    #[inline]
    fn hardware(&self) -> &axidma_pac::axi_dma::RegisterBlock {
        unsafe { &*(self.base_address as *const _) }
    }

    fn reset(&mut self) {
        let hardware: &axidma_pac::axi_dma::RegisterBlock =
            unsafe { &*(self.base_address as *const _) };
        if let Some(ring) = self.tx_bd_ring.as_mut() {
            if self.has_sg {
                // ring.snaphot_curr_bd();
            }
            hardware.mm2s_dmacr.modify(|_, w| w.reset().reset());
            ring.is_halted = true;
        }
        if let Some(ring) = self.rx_bd_ring.as_mut() {
            if self.has_sg {
                // ring.snaphot_curr_bd();
            }
            hardware.s2mm_dmacr.modify(|_, w| w.reset().reset());
            ring.is_halted = true;
        }
    }

    // reset is done when both went normal
    fn reset_is_done(&self) -> bool {
        if self.tx_bd_ring.is_some() && self.hardware().mm2s_dmacr.read().reset().is_reset() {
            return false;
        }
        if self.rx_bd_ring.is_some() && self.hardware().s2mm_dmacr.read().reset().is_reset() {
            return false;
        }
        true
    }

    fn start(&mut self) -> Result<(), ()> {
        if !self.is_initialized {
            error!("Start: Driver not initialized");
            return Err(());
        }
        let hardware: &axidma_pac::axi_dma::RegisterBlock =
            unsafe { &*(self.base_address as *const _) };
        if let Some(ring) = self.tx_bd_ring.as_mut() {
            if ring.is_halted {
                if self.has_sg {
                    ring.start().map_err(|e| {
                        error!("Start hw tx channel failed");
                        e
                    })?;
                } else {
                    compiler_fence(SeqCst);
                    fence(SeqCst);
                    io_fence();

                    hardware.mm2s_dmacr.modify(|_, w| w.run_stop().run())
                }
                ring.is_halted = false;
            }
        }
        if let Some(ring) = self.rx_bd_ring.as_mut() {
            if ring.is_halted {
                if self.has_sg {
                    ring.start().map_err(|e| {
                        error!("Start hw rx channel failed");
                        e
                    })?;
                } else {
                    compiler_fence(SeqCst);
                    fence(SeqCst);
                    io_fence();

                    hardware.s2mm_dmacr.modify(|_, w| w.run_stop().run())
                }
                ring.is_halted = false;
            }
        }
        Ok(())
    }

    pub fn pause(&mut self) -> Result<(), ()> {
        if !self.is_initialized {
            error!("Pause: Driver not initialized");
            return Err(());
        }
        let hardware: &axidma_pac::axi_dma::RegisterBlock =
            unsafe { &*(self.base_address as *const _) };
        if let Some(ring) = self.tx_bd_ring.as_mut() {
            if !self.has_sg {
                hardware.mm2s_dmacr.modify(|_, w| w.run_stop().stop())
            }
            ring.is_halted = true;
        }
        if let Some(ring) = self.rx_bd_ring.as_mut() {
            if !self.has_sg {
                hardware.s2mm_dmacr.modify(|_, w| w.run_stop().stop())
            }
            ring.is_halted = true;
        }
        Ok(())
    }

    pub fn resume(&mut self) -> Result<(), ()> {
        if !self.is_initialized {
            error!("Resume: Driver not initialized");
            return Err(());
        }
        self.start().map_err(|e| {
            error!("Resume: Failed to start engine");
            e
        })?;
        Ok(())
    }

    pub fn tx_intr_disable(&self) {
        debug!("axidma::tx_intr_disable");
        self.hardware().mm2s_dmacr.modify(|_, w| {
            w.dly_irq_en()
                .disable()
                .err_irq_en()
                .disable()
                .ioc_irq_en()
                .disable()
        })
    }

    pub fn rx_intr_disable(&self) {
        debug!("axidma::rx_intr_disable");
        self.hardware().s2mm_dmacr.modify(|_, w| {
            w.dly_irq_en()
                .disable()
                .err_irq_en()
                .disable()
                .ioc_irq_en()
                .disable()
        })
    }

    pub fn tx_intr_enable(&self) {
        debug!("axidma::tx_intr_enable");
        self.hardware().mm2s_dmacr.modify(|_, w| {
            w.dly_irq_en()
                .enable()
                .err_irq_en()
                .enable()
                .ioc_irq_en()
                .enable()
        })
    }

    pub fn rx_intr_enable(&self) {
        debug!("axidma::rx_intr_enable");
        self.hardware().s2mm_dmacr.modify(|_, w| {
            w.dly_irq_en()
                .enable()
                .err_irq_en()
                .enable()
                .ioc_irq_en()
                .enable()
        })
    }

    pub fn tx_bd_create(&mut self, bd_count: usize) {
        self.tx_intr_disable();
        if let Some(ring) = self.tx_bd_ring.as_mut() {
            ring.create(bd_count);
        }
    }

    pub fn rx_bd_create(&mut self, bd_count: usize) {
        self.rx_intr_disable();
        if let Some(ring) = self.rx_bd_ring.as_mut() {
            ring.create(bd_count);
        }
    }

    pub fn tx_submit<B>(&mut self, bufs: &[Pin<B>])
    where
        B: Deref,
        B::Target: AsRef<[u8]>,
    {
        if let Some(ring) = self.tx_bd_ring.as_mut() {
            ring.submit(bufs);
        } else {
            warn!("axidma::tx_submit: no tx ring!");
        }
    }

    pub fn rx_submit<B>(&mut self, bufs: &[Pin<B>])
    where
        B: Deref,
        B::Target: AsRef<[u8]>,
    {
        if let Some(ring) = self.rx_bd_ring.as_mut() {
            ring.submit(bufs);
        } else {
            warn!("axidma::rx_submit: no rx ring!");
        }
    }

    pub fn tx_to_hw(&mut self) {
        let hardware: &axidma_pac::axi_dma::RegisterBlock =
            unsafe { &*(self.base_address as *const _) };
        if let Some(ring) = self.tx_bd_ring.as_mut() {
            if ring.is_halted {
                // update cur desc
                let addr = ring.head_desc_addr();
                let addr_lsb = ((addr & 0xFFFF_FFFF) >> 6) as _;
                let addr_msb = (addr >> 32) as _;
                debug!("axidma::tx_to_hw: cur desc addr: 0x{:x}", addr);
                unsafe {
                    hardware
                        .mm2s_curdesc
                        .write(|w| w.curdesc_ptr().bits(addr_lsb));
                    hardware
                        .mm2s_curdesc_msb
                        .write(|w| w.curdesc_ptr().bits(addr_msb));
                }
            } else {
                warn!("axidma::tx_to_hw: ring running, cur desc not updated");
            }
            compiler_fence(SeqCst);
            fence(SeqCst);
            io_fence();

            hardware.mm2s_dmacr.modify(|_, w| w.run_stop().run());
            ring.is_halted = false;
            if ring.pending_cnt > 0 {
                ring.submit_cnt += ring.pending_cnt;
                ring.pending_cnt = 0;
                // update tail desc
                let addr = ring.tail_desc_addr();
                let addr_lsb = ((addr & 0xFFFF_FFFF) >> 6) as _;
                let addr_msb = (addr >> 32) as _;
                debug!("axidma::tx_to_hw: tail desc addr: 0x{:x}", addr);
                unsafe {
                    hardware
                        .mm2s_taildesc
                        .write(|w| w.taildesc_ptr().bits(addr_lsb));
                    hardware
                        .mm2s_taildesc_msb
                        .write(|w| w.taildesc_ptr().bits(addr_msb));
                }
            } else {
                warn!("axidma::tx_to_hw: no pending BD, tail desc not updated");
            }
        } else {
            warn!("axidma::tx_to_hw: no tx ring!");
        }
    }

    pub fn rx_to_hw(&mut self) {
        let hardware: &axidma_pac::axi_dma::RegisterBlock =
            unsafe { &*(self.base_address as *const _) };
        if let Some(ring) = self.rx_bd_ring.as_mut() {
            if ring.is_halted {
                // update cur desc
                let addr = ring.head_desc_addr();
                let addr_lsb = ((addr & 0xFFFF_FFFF) >> 6) as _;
                let addr_msb = (addr >> 32) as _;
                debug!("axidma::rx_to_hw: cur desc addr: 0x{:x}", addr);

                unsafe {
                    hardware
                        .s2mm_curdesc
                        .write(|w| w.curdesc_ptr().bits(addr_lsb));
                    hardware
                        .s2mm_curdesc_msb
                        .write(|w| w.curdesc_ptr().bits(addr_msb));
                }
            } else {
                warn!("axidma::rx_to_hw: ring running, cur desc not updated");
            }

            compiler_fence(SeqCst);
            fence(SeqCst);
            io_fence();
            hardware.s2mm_dmacr.modify(|_, w| w.run_stop().run());
            ring.is_halted = false;
            if ring.pending_cnt > 0 {
                ring.submit_cnt += ring.pending_cnt;
                ring.pending_cnt = 0;
                // update tail desc
                let addr = ring.tail_desc_addr();
                let addr_lsb = ((addr & 0xFFFF_FFFF) >> 6) as _;
                let addr_msb = (addr >> 32) as _;
                debug!("axidma::rx_to_hw: tail desc addr: 0x{:x}", addr);
                unsafe {
                    hardware
                        .s2mm_taildesc
                        .write(|w| w.taildesc_ptr().bits(addr_lsb));
                    hardware
                        .s2mm_taildesc_msb
                        .write(|w| w.taildesc_ptr().bits(addr_msb));
                }
            } else {
                warn!("axidma::rx_to_hw: no pending BD, tail desc not updated");
            }
        } else {
            warn!("axidma::rx_to_hw: no rx ring!");
        }
    }

    pub fn tx_from_hw(&mut self) -> Option<Vec<Pin<&[u8]>>> {
        if let Some(ring) = self.tx_bd_ring.as_mut() {
            ring.from_hw()
        } else {
            warn!("axidma::tx_from_hw: no tx ring!");
            None
        }
    }

    pub fn rx_from_hw(&mut self) -> Option<Vec<Pin<&[u8]>>> {
        if let Some(ring) = self.rx_bd_ring.as_mut() {
            ring.from_hw()
        } else {
            warn!("axidma::rx_from_hw: no rx ring!");
            None
        }
    }
}

impl AxiDmaIntr {
    pub fn new(base_address: usize) -> Self {
        Self { base_address }
    }

    #[inline]
    fn hardware(&self) -> &axidma_pac::axi_dma::RegisterBlock {
        unsafe { &*(self.base_address as *const _) }
    }

    pub fn tx_intr_handler(&self) {
        let sr = &self.hardware().mm2s_dmasr;
        if sr.read().err_irq().is_detected() {
            // dump regs
            // reset
            error!("axidma_intr: tx err intr detected");
            self.tx_dump_regs();
            sr.modify(|_, w| w.err_irq().set_bit());
        }
        if sr.read().ioc_irq().is_detected() {
            debug!("axidma_intr: tx cplt intr detected");
            sr.modify(|_, w| w.ioc_irq().set_bit());
            TX_FRAMES.fetch_add(1, Relaxed);
        }
        if sr.read().dly_irq().is_detected() {
            debug!("axidma_intr: tx dly intr detected");
            sr.modify(|_, w| w.dly_irq().set_bit());
            TX_FRAMES.fetch_add(1, Relaxed);
        }
    }

    pub fn rx_intr_handler(&self) {
        let sr = &self.hardware().s2mm_dmasr;
        if sr.read().err_irq().is_detected() {
            // dump regs
            // reset
            error!("axidma: rx err intr detected");
            self.rx_dump_regs();
            sr.modify(|_, w| w.err_irq().set_bit());
        }
        if sr.read().ioc_irq().is_detected() {
            debug!("axidma_intr: rx cplt intr detected");
            sr.modify(|_, w| w.ioc_irq().set_bit());
            RX_FRAMES.fetch_add(1, Relaxed);
        }
        if sr.read().dly_irq().is_detected() {
            debug!("axidma_intr: rx dly intr detected");
            sr.modify(|_, w| w.dly_irq().set_bit());
            RX_FRAMES.fetch_add(1, Relaxed);
        }
    }

    fn tx_dump_regs(&self) {
        let hw = self.hardware();
        info!(
            "CR: 0x{:x}, SR: 0x{:x}",
            hw.mm2s_dmacr.read().bits(),
            hw.mm2s_dmasr.read().bits()
        );
        info!(
            "CDESC_MSB: 0x{:x}, CDESC: 0x{:x}",
            hw.mm2s_curdesc_msb.read().bits(),
            hw.mm2s_curdesc_msb.read().bits()
        );
        info!(
            "TDESC_MSB: 0x{:x}, TDESC: 0x{:x}",
            hw.mm2s_taildesc_msb.read().bits(),
            hw.mm2s_taildesc.read().bits()
        );
    }

    fn rx_dump_regs(&self) {
        let hw = self.hardware();
        info!(
            "CR: 0x{:x}, SR: 0x{:x}",
            hw.s2mm_dmacr.read().bits(),
            hw.s2mm_dmasr.read().bits()
        );
        info!(
            "CDESC_MSB: 0x{:x}, CDESC: 0x{:x}",
            hw.s2mm_curdesc_msb.read().bits(),
            hw.s2mm_curdesc_msb.read().bits()
        );
        info!(
            "TDESC_MSB: 0x{:x}, TDESC: 0x{:x}",
            hw.s2mm_taildesc_msb.read().bits(),
            hw.s2mm_taildesc.read().bits()
        );
    }
}

#[inline]
pub fn io_fence() {
    unsafe {
        asm!("fence iorw,iorw");
    }
}

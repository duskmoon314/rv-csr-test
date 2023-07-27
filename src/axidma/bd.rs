use core::slice::from_raw_parts;

use axidma_pac::sg_desc::RegisterBlock;
#[repr(C, align(64))]
pub struct AxiDmaBlockDesc {
    pub desc: RegisterBlock,
    pub sw_id: u32,
    pub has_sts_cntrl: bool,
    pub has_dre: bool,
    pub word_len: u32,
}

impl AxiDmaBlockDesc {
    pub fn new(has_sts_cntrl: bool, has_dre: bool, word_len: u32) -> Self {
        Self {
            desc: RegisterBlock::default(),
            sw_id: 0,
            has_sts_cntrl,
            has_dre,
            word_len,
        }
    }

    pub fn clear(&self) {
        self.desc.buf_addr.reset();
        self.desc.buf_addr_msb.reset();
        self.desc.control.reset();
        for app in self.desc.app.iter() {
            app.reset();
        }
    }

    pub fn set_next_desc_addr(&self, addr: usize) {
        let addr_lsb = ((addr & 0xFFFF_FFFF) >> 6) as _;
        let addr_msb = (addr >> 32) as _;
        unsafe {
            self.desc
                .nxt_desc
                .write(|w| w.nxt_desc_ptr().bits(addr_lsb));
            self.desc
                .nxt_desc_msb
                .write(|w| w.nxt_desc_ptr().bits(addr_msb));
        }
    }

    pub fn buf(&self) -> &[u8] {
        let addr_lsb = self.desc.buf_addr.read().buf_addr().bits() as usize;
        let addr_msb = self.desc.buf_addr_msb.read().buf_addr().bits() as usize;
        let addr = (addr_msb << 32) | addr_lsb;
        let len = self.desc.control.read().buf_len().bits() as _;
        unsafe { from_raw_parts(addr as _, len) }
    }

    pub fn set_buf(&self, buf: &[u8]) {
        let addr = buf.as_ptr() as usize;
        let addr_lsb = (addr & 0xFFFF_FFFF) as _;
        let addr_msb = (addr >> 32) as _;
        debug!("bd::set_buf: addr: {:x}, len: {}", addr, buf.len());
        unsafe {
            self.desc.buf_addr.write(|w| w.buf_addr().bits(addr_lsb));
            self.desc
                .buf_addr_msb
                .write(|w| w.buf_addr().bits(addr_msb));
            self.desc
                .control
                .modify(|_, w| w.buf_len().bits(buf.len() as _));
        }
    }

    pub fn dump(&self) {
        let d = &self.desc;
        info!(
            "NXT_DESC_MSB: 0x{:x}, NXT_DESC: 0x{:x}",
            d.nxt_desc_msb.read().bits(),
            d.nxt_desc.read().bits()
        );
        info!(
            "BUF_ADDR_MSB: 0x{:x}, BUF_ADDR: 0x{:x}",
            d.buf_addr_msb.read().bits(),
            d.buf_addr.read().bits()
        );
        info!(
            "CONTROL: 0x{:x}, STATUS: 0x{:x}",
            d.control.read().bits(),
            d.status.read().bits()
        );
    }
}

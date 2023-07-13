#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MM2S DMA Control register"]
    pub mm2s_dmacr: MM2S_DMACR,
    #[doc = "0x04 - MM2S DMA Status register"]
    pub mm2s_dmasr: MM2S_DMASR,
    #[doc = "0x08 - MM2S Current Descriptor Pointer. Lower 32 bits of the address."]
    pub mm2s_curdesc: MM2S_CURDESC,
    #[doc = "0x0c - MM2S Current Descriptor Pointer. Upper 32 bits of the address."]
    pub mm2s_curdesc_msb: MM2S_CURDESC_MSB,
    #[doc = "0x10 - MM2S Tail Descriptor Pointer. Lower 32 bits of the address."]
    pub mm2s_taildesc: MM2S_TAILDESC,
    #[doc = "0x14 - MM2S Tail Descriptor Pointer. Upper 32 bits of the address."]
    pub mm2s_taildesc_msb: MM2S_TAILDESC_MSB,
    #[doc = "0x18 - MM2S Source Address. Lower 32 bits of the address."]
    pub mm2s_sa: MM2S_SA,
    #[doc = "0x1c - MM2S Source Address. Upper 32 bits of the address."]
    pub mm2s_sa_msb: MM2S_SA_MSB,
    _reserved8: [u8; 0x08],
    #[doc = "0x28 - MM2S Transfer Length (Bytes)"]
    pub mm2s_length: MM2S_LENGTH,
    #[doc = "0x2c - Scatter/Gather User and Cache"]
    pub sg_ctl: SG_CTL,
    #[doc = "0x30 - S2MM DMA Control register"]
    pub s2mm_dmacr: S2MM_DMACR,
    #[doc = "0x34 - S2MM DMA Status register"]
    pub s2mm_dmasr: S2MM_DMASR,
    #[doc = "0x38 - S2MM Current Descriptor Pointer. Lower 32 address bits."]
    pub s2mm_curdesc: S2MM_CURDESC,
    #[doc = "0x3c - S2MM Current Descriptor Pointer. Upper 32 address bits."]
    pub s2mm_curdesc_msb: S2MM_CURDESC_MSB,
    #[doc = "0x40 - S2MM Tail Descriptor Pointer. Lower 32 address bits."]
    pub s2mm_taildesc: S2MM_TAILDESC,
    #[doc = "0x44 - S2MM Tail Descriptor Pointer. Upper 32 address bits."]
    pub s2mm_taildesc_msb: S2MM_TAILDESC_MSB,
    #[doc = "0x48 - S2MM Destination Address. Lower 32 bit address"]
    pub s2mm_da: S2MM_DA,
    #[doc = "0x4c - S2MM Destination Address. Upper 32 bit address."]
    pub s2mm_da_msb: S2MM_DA_MSB,
    _reserved18: [u8; 0x08],
    #[doc = "0x58 - S2MM Buffer Length (Bytes)"]
    pub s2mm_length: S2MM_LENGTH,
}
#[doc = "mm2s_dmacr (rw) register accessor: an alias for `Reg<MM2S_DMACR_SPEC>`"]
pub type MM2S_DMACR = crate::Reg<mm2s_dmacr::MM2S_DMACR_SPEC>;
#[doc = "MM2S DMA Control register"]
pub mod mm2s_dmacr;
#[doc = "mm2s_dmasr (rw) register accessor: an alias for `Reg<MM2S_DMASR_SPEC>`"]
pub type MM2S_DMASR = crate::Reg<mm2s_dmasr::MM2S_DMASR_SPEC>;
#[doc = "MM2S DMA Status register"]
pub mod mm2s_dmasr;
#[doc = "mm2s_curdesc (rw) register accessor: an alias for `Reg<MM2S_CURDESC_SPEC>`"]
pub type MM2S_CURDESC = crate::Reg<mm2s_curdesc::MM2S_CURDESC_SPEC>;
#[doc = "MM2S Current Descriptor Pointer. Lower 32 bits of the address."]
pub mod mm2s_curdesc;
#[doc = "mm2s_curdesc_msb (rw) register accessor: an alias for `Reg<MM2S_CURDESC_MSB_SPEC>`"]
pub type MM2S_CURDESC_MSB = crate::Reg<mm2s_curdesc_msb::MM2S_CURDESC_MSB_SPEC>;
#[doc = "MM2S Current Descriptor Pointer. Upper 32 bits of the address."]
pub mod mm2s_curdesc_msb;
#[doc = "mm2s_taildesc (rw) register accessor: an alias for `Reg<MM2S_TAILDESC_SPEC>`"]
pub type MM2S_TAILDESC = crate::Reg<mm2s_taildesc::MM2S_TAILDESC_SPEC>;
#[doc = "MM2S Tail Descriptor Pointer. Lower 32 bits of the address."]
pub mod mm2s_taildesc;
#[doc = "mm2s_taildesc_msb (rw) register accessor: an alias for `Reg<MM2S_TAILDESC_MSB_SPEC>`"]
pub type MM2S_TAILDESC_MSB = crate::Reg<mm2s_taildesc_msb::MM2S_TAILDESC_MSB_SPEC>;
#[doc = "MM2S Tail Descriptor Pointer. Upper 32 bits of the address."]
pub mod mm2s_taildesc_msb;
#[doc = "mm2s_sa (rw) register accessor: an alias for `Reg<MM2S_SA_SPEC>`"]
pub type MM2S_SA = crate::Reg<mm2s_sa::MM2S_SA_SPEC>;
#[doc = "MM2S Source Address. Lower 32 bits of the address."]
pub mod mm2s_sa;
#[doc = "mm2s_sa_msb (rw) register accessor: an alias for `Reg<MM2S_SA_MSB_SPEC>`"]
pub type MM2S_SA_MSB = crate::Reg<mm2s_sa_msb::MM2S_SA_MSB_SPEC>;
#[doc = "MM2S Source Address. Upper 32 bits of the address."]
pub mod mm2s_sa_msb;
#[doc = "mm2s_length (rw) register accessor: an alias for `Reg<MM2S_LENGTH_SPEC>`"]
pub type MM2S_LENGTH = crate::Reg<mm2s_length::MM2S_LENGTH_SPEC>;
#[doc = "MM2S Transfer Length (Bytes)"]
pub mod mm2s_length;
#[doc = "sg_ctl (rw) register accessor: an alias for `Reg<SG_CTL_SPEC>`"]
pub type SG_CTL = crate::Reg<sg_ctl::SG_CTL_SPEC>;
#[doc = "Scatter/Gather User and Cache"]
pub mod sg_ctl;
#[doc = "s2mm_dmacr (rw) register accessor: an alias for `Reg<S2MM_DMACR_SPEC>`"]
pub type S2MM_DMACR = crate::Reg<s2mm_dmacr::S2MM_DMACR_SPEC>;
#[doc = "S2MM DMA Control register"]
pub mod s2mm_dmacr;
#[doc = "s2mm_dmasr (rw) register accessor: an alias for `Reg<S2MM_DMASR_SPEC>`"]
pub type S2MM_DMASR = crate::Reg<s2mm_dmasr::S2MM_DMASR_SPEC>;
#[doc = "S2MM DMA Status register"]
pub mod s2mm_dmasr;
#[doc = "s2mm_curdesc (rw) register accessor: an alias for `Reg<S2MM_CURDESC_SPEC>`"]
pub type S2MM_CURDESC = crate::Reg<s2mm_curdesc::S2MM_CURDESC_SPEC>;
#[doc = "S2MM Current Descriptor Pointer. Lower 32 address bits."]
pub mod s2mm_curdesc;
#[doc = "s2mm_curdesc_msb (rw) register accessor: an alias for `Reg<S2MM_CURDESC_MSB_SPEC>`"]
pub type S2MM_CURDESC_MSB = crate::Reg<s2mm_curdesc_msb::S2MM_CURDESC_MSB_SPEC>;
#[doc = "S2MM Current Descriptor Pointer. Upper 32 address bits."]
pub mod s2mm_curdesc_msb;
#[doc = "s2mm_taildesc (rw) register accessor: an alias for `Reg<S2MM_TAILDESC_SPEC>`"]
pub type S2MM_TAILDESC = crate::Reg<s2mm_taildesc::S2MM_TAILDESC_SPEC>;
#[doc = "S2MM Tail Descriptor Pointer. Lower 32 address bits."]
pub mod s2mm_taildesc;
#[doc = "s2mm_taildesc_msb (rw) register accessor: an alias for `Reg<S2MM_TAILDESC_MSB_SPEC>`"]
pub type S2MM_TAILDESC_MSB = crate::Reg<s2mm_taildesc_msb::S2MM_TAILDESC_MSB_SPEC>;
#[doc = "S2MM Tail Descriptor Pointer. Upper 32 address bits."]
pub mod s2mm_taildesc_msb;
#[doc = "s2mm_da (rw) register accessor: an alias for `Reg<S2MM_DA_SPEC>`"]
pub type S2MM_DA = crate::Reg<s2mm_da::S2MM_DA_SPEC>;
#[doc = "S2MM Destination Address. Lower 32 bit address"]
pub mod s2mm_da;
#[doc = "s2mm_da_msb (rw) register accessor: an alias for `Reg<S2MM_DA_MSB_SPEC>`"]
pub type S2MM_DA_MSB = crate::Reg<s2mm_da_msb::S2MM_DA_MSB_SPEC>;
#[doc = "S2MM Destination Address. Upper 32 bit address."]
pub mod s2mm_da_msb;
#[doc = "s2mm_length (rw) register accessor: an alias for `Reg<S2MM_LENGTH_SPEC>`"]
pub type S2MM_LENGTH = crate::Reg<s2mm_length::S2MM_LENGTH_SPEC>;
#[doc = "S2MM Buffer Length (Bytes)"]
pub mod s2mm_length;

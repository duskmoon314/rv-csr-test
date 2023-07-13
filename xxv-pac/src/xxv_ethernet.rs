#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GT Reset Register"]
    pub grr: GRR,
    #[doc = "0x04 - Reset Register"]
    pub rst: RST,
    #[doc = "0x08 - Mode Register"]
    pub mode: MODE,
    #[doc = "0x0c - Tx Configuration Register"]
    pub txcfg: TXCFG,
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - Rx Configuration Register"]
    pub rxcfg: RXCFG,
    #[doc = "0x18 - Rx MTU Configuration Register"]
    pub rxmtu: RXMTU,
    #[doc = "0x1c - VL Length Configuration Register"]
    pub vll: VLL,
    #[doc = "0x20 - TICK Register"]
    pub tick: TICK,
    #[doc = "0x24 - Revision Register"]
    pub rev: REV,
    _reserved9: [u8; 0xa8],
    #[doc = "0xd0 - Refec Register"]
    pub rsfec: RSFEC,
    #[doc = "0xd4 - Fec Register"]
    pub fec: FEC,
    _reserved11: [u8; 0x08],
    #[doc = "0xe0 - AN_CONTROL1 Configuration Register"]
    pub ancr1: ANCR1,
    #[doc = "0xe4 - AN_CONTROL2 Configuration Register"]
    pub ancr2: ANCR2,
    _reserved13: [u8; 0x10],
    #[doc = "0xf8 - AN_ABILITY Configuration Register"]
    pub anacr: ANACR,
    _reserved14: [u8; 0x04],
    #[doc = "0x100 - LT_CONTROL Configuration Register"]
    pub ltcr: LTCR,
    #[doc = "0x104 - LT_TRAINED Configuration Register"]
    pub lttr: LTTR,
    #[doc = "0x108 - LT_PRESET Configuration Register"]
    pub ltpr: LTPR,
    #[doc = "0x10c - LT_INIT Configuration Register"]
    pub ltir: LTIR,
    #[doc = "0x110 - LT_SEED Configuration Register"]
    pub ltsr: LTSR,
    _reserved19: [u8; 0x1c],
    #[doc = "0x130 - LT_COEFFICIENT Configuration Register"]
    pub ltcor: LTCOR,
    _reserved20: [u8; 0x02cc],
    #[doc = "0x400 - Stat TX Status Register"]
    pub txsr: TXSR,
    #[doc = "0x404 - Stat RX Status Register"]
    pub rxsr: RXSR,
    #[doc = "0x408 - Stat Status Register"]
    pub sr: SR,
    #[doc = "0x40c - Stat RX Block Lock Register"]
    pub rxblsr: RXBLSR,
    _reserved24: [u8; 0x48],
    #[doc = "0x458 - Stat AN_STATUS Register"]
    pub ansr: ANSR,
    #[doc = "0x45c - Stat AN_ABILITY Register"]
    pub anasr: ANASR,
}
#[doc = "grr (rw) register accessor: an alias for `Reg<GRR_SPEC>`"]
pub type GRR = crate::Reg<grr::GRR_SPEC>;
#[doc = "GT Reset Register"]
pub mod grr;
#[doc = "rst (rw) register accessor: an alias for `Reg<RST_SPEC>`"]
pub type RST = crate::Reg<rst::RST_SPEC>;
#[doc = "Reset Register"]
pub mod rst;
#[doc = "mode (rw) register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Mode Register"]
pub mod mode;
#[doc = "txcfg (rw) register accessor: an alias for `Reg<TXCFG_SPEC>`"]
pub type TXCFG = crate::Reg<txcfg::TXCFG_SPEC>;
#[doc = "Tx Configuration Register"]
pub mod txcfg;
#[doc = "rxcfg (rw) register accessor: an alias for `Reg<RXCFG_SPEC>`"]
pub type RXCFG = crate::Reg<rxcfg::RXCFG_SPEC>;
#[doc = "Rx Configuration Register"]
pub mod rxcfg;
#[doc = "rxmtu (rw) register accessor: an alias for `Reg<RXMTU_SPEC>`"]
pub type RXMTU = crate::Reg<rxmtu::RXMTU_SPEC>;
#[doc = "Rx MTU Configuration Register"]
pub mod rxmtu;
#[doc = "vll (rw) register accessor: an alias for `Reg<VLL_SPEC>`"]
pub type VLL = crate::Reg<vll::VLL_SPEC>;
#[doc = "VL Length Configuration Register"]
pub mod vll;
#[doc = "tick (w) register accessor: an alias for `Reg<TICK_SPEC>`"]
pub type TICK = crate::Reg<tick::TICK_SPEC>;
#[doc = "TICK Register"]
pub mod tick;
#[doc = "rev (r) register accessor: an alias for `Reg<REV_SPEC>`"]
pub type REV = crate::Reg<rev::REV_SPEC>;
#[doc = "Revision Register"]
pub mod rev;
#[doc = "rsfec (rw) register accessor: an alias for `Reg<RSFEC_SPEC>`"]
pub type RSFEC = crate::Reg<rsfec::RSFEC_SPEC>;
#[doc = "Refec Register"]
pub mod rsfec;
#[doc = "fec (rw) register accessor: an alias for `Reg<FEC_SPEC>`"]
pub type FEC = crate::Reg<fec::FEC_SPEC>;
#[doc = "Fec Register"]
pub mod fec;
#[doc = "ancr1 (rw) register accessor: an alias for `Reg<ANCR1_SPEC>`"]
pub type ANCR1 = crate::Reg<ancr1::ANCR1_SPEC>;
#[doc = "AN_CONTROL1 Configuration Register"]
pub mod ancr1;
#[doc = "ancr2 (rw) register accessor: an alias for `Reg<ANCR2_SPEC>`"]
pub type ANCR2 = crate::Reg<ancr2::ANCR2_SPEC>;
#[doc = "AN_CONTROL2 Configuration Register"]
pub mod ancr2;
#[doc = "anacr (rw) register accessor: an alias for `Reg<ANACR_SPEC>`"]
pub type ANACR = crate::Reg<anacr::ANACR_SPEC>;
#[doc = "AN_ABILITY Configuration Register"]
pub mod anacr;
#[doc = "ltcr (rw) register accessor: an alias for `Reg<LTCR_SPEC>`"]
pub type LTCR = crate::Reg<ltcr::LTCR_SPEC>;
#[doc = "LT_CONTROL Configuration Register"]
pub mod ltcr;
#[doc = "lttr (rw) register accessor: an alias for `Reg<LTTR_SPEC>`"]
pub type LTTR = crate::Reg<lttr::LTTR_SPEC>;
#[doc = "LT_TRAINED Configuration Register"]
pub mod lttr;
#[doc = "ltpr (rw) register accessor: an alias for `Reg<LTPR_SPEC>`"]
pub type LTPR = crate::Reg<ltpr::LTPR_SPEC>;
#[doc = "LT_PRESET Configuration Register"]
pub mod ltpr;
#[doc = "ltir (rw) register accessor: an alias for `Reg<LTIR_SPEC>`"]
pub type LTIR = crate::Reg<ltir::LTIR_SPEC>;
#[doc = "LT_INIT Configuration Register"]
pub mod ltir;
#[doc = "ltsr (rw) register accessor: an alias for `Reg<LTSR_SPEC>`"]
pub type LTSR = crate::Reg<ltsr::LTSR_SPEC>;
#[doc = "LT_SEED Configuration Register"]
pub mod ltsr;
#[doc = "ltcor (rw) register accessor: an alias for `Reg<LTCOR_SPEC>`"]
pub type LTCOR = crate::Reg<ltcor::LTCOR_SPEC>;
#[doc = "LT_COEFFICIENT Configuration Register"]
pub mod ltcor;
#[doc = "txsr (r) register accessor: an alias for `Reg<TXSR_SPEC>`"]
pub type TXSR = crate::Reg<txsr::TXSR_SPEC>;
#[doc = "Stat TX Status Register"]
pub mod txsr;
#[doc = "rxsr (r) register accessor: an alias for `Reg<RXSR_SPEC>`"]
pub type RXSR = crate::Reg<rxsr::RXSR_SPEC>;
#[doc = "Stat RX Status Register"]
pub mod rxsr;
#[doc = "sr (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Stat Status Register"]
pub mod sr;
#[doc = "rxblsr (r) register accessor: an alias for `Reg<RXBLSR_SPEC>`"]
pub type RXBLSR = crate::Reg<rxblsr::RXBLSR_SPEC>;
#[doc = "Stat RX Block Lock Register"]
pub mod rxblsr;
#[doc = "ansr (r) register accessor: an alias for `Reg<ANSR_SPEC>`"]
pub type ANSR = crate::Reg<ansr::ANSR_SPEC>;
#[doc = "Stat AN_STATUS Register"]
pub mod ansr;
#[doc = "anasr (r) register accessor: an alias for `Reg<ANASR_SPEC>`"]
pub type ANASR = crate::Reg<anasr::ANASR_SPEC>;
#[doc = "Stat AN_ABILITY Register"]
pub mod anasr;

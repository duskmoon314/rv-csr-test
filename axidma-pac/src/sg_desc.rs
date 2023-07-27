#[doc = r"Register block"]
#[repr(C)]
#[derive(Default)]
pub struct RegisterBlock {
    #[doc = "0x00 - Next Descriptor Pointer"]
    pub nxt_desc: NXT_DESC,
    #[doc = "0x04 - Upper 32 bits of Next Descriptor Pointer"]
    pub nxt_desc_msb: NXT_DESC_MSB,
    #[doc = "0x08 - Buffer Address"]
    pub buf_addr: BUF_ADDR,
    #[doc = "0x0c - Upper 32 bits of Buffer Address"]
    pub buf_addr_msb: BUF_ADDR_MSB,
    _reserved4: [u8; 0x08],
    #[doc = "0x18 - "]
    pub control: CONTROL,
    #[doc = "0x1c - "]
    pub status: STATUS,
    #[doc = "0x20..0x34 - User Application Field \\[%s\\]"]
    pub app: [APP; 5],
}
#[doc = "nxt_desc (rw) register accessor: an alias for `Reg<NXT_DESC_SPEC>`"]
pub type NXT_DESC = crate::Reg<nxt_desc::NXT_DESC_SPEC>;
#[doc = "Next Descriptor Pointer"]
pub mod nxt_desc;
#[doc = "nxt_desc_msb (rw) register accessor: an alias for `Reg<NXT_DESC_MSB_SPEC>`"]
pub type NXT_DESC_MSB = crate::Reg<nxt_desc_msb::NXT_DESC_MSB_SPEC>;
#[doc = "Upper 32 bits of Next Descriptor Pointer"]
pub mod nxt_desc_msb;
#[doc = "buf_addr (rw) register accessor: an alias for `Reg<BUF_ADDR_SPEC>`"]
pub type BUF_ADDR = crate::Reg<buf_addr::BUF_ADDR_SPEC>;
#[doc = "Buffer Address"]
pub mod buf_addr;
#[doc = "buf_addr_msb (rw) register accessor: an alias for `Reg<BUF_ADDR_MSB_SPEC>`"]
pub type BUF_ADDR_MSB = crate::Reg<buf_addr_msb::BUF_ADDR_MSB_SPEC>;
#[doc = "Upper 32 bits of Buffer Address"]
pub mod buf_addr_msb;
#[doc = "control (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = ""]
pub mod control;
#[doc = "status (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = ""]
pub mod status;
#[doc = "app (rw) register accessor: an alias for `Reg<APP_SPEC>`"]
pub type APP = crate::Reg<app::APP_SPEC>;
#[doc = "User Application Field \\[%s\\]"]
pub mod app;

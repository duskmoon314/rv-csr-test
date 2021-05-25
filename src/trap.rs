use riscv::register::{
    mtvec::TrapMode,
    scause::{self},
    sepc, sip,
    sstatus::Sstatus,
    stval, stvec, ucause, uepc, uip,
    ustatus::{self, Ustatus},
    utval, utvec,
};

use crate::sbi;

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}

#[repr(C)]
pub struct UserTrapContext {
    pub x: [usize; 32],
    pub ustatus: Ustatus,
    pub uepc: usize,
}

impl UserTrapContext {
    pub fn init(entry: usize, sp: usize, s: [usize; 12]) -> Self {
        let mut ustatus = ustatus::read();
        ustatus.set_upie(true);
        let mut cx = Self {
            x: [0; 32],
            ustatus,
            uepc: entry,
        };
        cx.x[2] = sp;
        cx.x[8] = s[0];
        cx.x[9] = s[1];
        for i in 18..=27 {
            cx.x[i] = s[i - 16];
        }

        cx
    }
}

global_asm!(include_str!("trap.asm"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

pub fn init_u() {
    extern "C" {
        fn __alltraps_u();
    }
    unsafe {
        utvec::write(__alltraps_u as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        // scause::Trap::Exception(scause::Exception::UserEnvCall) => {
        //     cx.sepc += 4;
        //     cx.x[10] = sbi::sbi_call(cx.x[17], cx.x[10], cx.x[11], cx.x[12]) as usize;
        // }
        scause::Trap::Interrupt(scause::Interrupt::UserSoft) => {
            debug!("user soft in supervisor");
            unsafe {
                sip::clear_usoft();
            }
        }
        scause::Trap::Interrupt(scause::Interrupt::SupervisorSoft) => {
            debug!("supervisor soft");
            unsafe {
                sip::clear_ssoft();
            }
        }
        _ => {
            error!(
                "Unsupported trap {:?}, stval = {:#x}, sepc = {:#x}!",
                scause.cause(),
                stval,
                sepc::read()
            );
            loop {}
        }
    }
    cx
}

#[no_mangle]
pub fn user_trap_handler(cx: &mut UserTrapContext) -> &mut UserTrapContext {
    let ucause = ucause::read();
    let utval = utval::read();
    match ucause.cause() {
        ucause::Trap::Interrupt(ucause::Interrupt::UserSoft) => {
            // debug!("user soft");
            unsafe {
                uip::clear_usoft();
            }
        }
        _ => {
            error!(
                "Unsupported trap {:?}, utval = {:#x}, uepc = {:#x}!",
                ucause.cause(),
                utval,
                uepc::read()
            );
        }
    }
    cx
}

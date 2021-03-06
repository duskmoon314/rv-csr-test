use crate::trap::{TrapContext, UserTrapContext};

#[allow(unused)]
const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;

#[repr(align(4096))]
pub struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
pub struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

pub static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};

pub static USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

impl UserStack {
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
    pub fn push_context(&self, cx: UserTrapContext) -> &'static mut UserTrapContext {
        let cx_ptr =
            (self.get_sp() - core::mem::size_of::<UserTrapContext>()) as *mut UserTrapContext;
        unsafe {
            *cx_ptr = cx;
        }
        unsafe { cx_ptr.as_mut().unwrap() }
    }
}

impl KernelStack {
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
        }
        unsafe { cx_ptr.as_mut().unwrap() }
    }
    pub fn push_ucontext(&self, cx: UserTrapContext) -> &'static mut UserTrapContext {
        let cx_ptr =
            (self.get_sp() - core::mem::size_of::<UserTrapContext>()) as *mut UserTrapContext;
        unsafe {
            *cx_ptr = cx;
        }
        unsafe { cx_ptr.as_mut().unwrap() }
    }
}

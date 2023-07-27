use crate::generic::{Reg, RegisterSpec};
use core::marker;

impl<REG: RegisterSpec> Default for Reg<REG> {
    fn default() -> Self {
        Self {
            register: vcell::VolatileCell::new(REG::Ux::from(false)),
            _marker: marker::PhantomData,
        }
    }
}

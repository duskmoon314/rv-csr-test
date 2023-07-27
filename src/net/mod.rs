#[cfg(feature = "board_qemu")]
mod virtio_net;
#[cfg(feature = "board_qemu")]
pub use virtio_net::*;

// #[cfg(feature = "board_lrv")]
mod xxvethernet;
// #[cfg(feature = "board_lrv")]
pub use xxvethernet::*;

pub fn init() {
    #[cfg(feature = "board_qemu")]
    virtio_net::init();
    #[cfg(feature = "board_lrv")]
    xxvethernet::init();
}

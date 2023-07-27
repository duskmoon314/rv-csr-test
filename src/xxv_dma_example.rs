use alloc::boxed::Box;
use core::sync::atomic::Ordering::Relaxed;
use riscv::register::{sie, uie};

use crate::{
    axidma::{AXI_DMA, AXI_DMA_INTR, RX_FRAMES, TX_FRAMES},
    net::{XXE_HDR_SIZE, XXE_MAC_ADDR_SIZE, XXE_MAX_JUMBO_FRAME_SIZE, XXV_ETHERNET},
    plic::{self, Plic},
    HAS_INTR,
};

const RX_BD_CNT: usize = 1024;
const TX_BD_CNT: usize = 1024;

// const MAC_ADDR: [u8; XXE_MAC_ADDR_SIZE] = [0x00, 0x00, 0x53, 0x0e, 0x9f, 0xb0];
const SRC_MAC_ADDR: [u8; XXE_MAC_ADDR_SIZE] = [0x00, 0x0A, 0x35, 0x01, 0x02, 0x03];
const DEST_MAC_ADDR: [u8; XXE_MAC_ADDR_SIZE] = [0x00, 0x16, 0x31, 0xf3, 0xc9, 0xad];
const PAYLOAD_SIZE: usize = 8900;

pub fn xxv_dma_example(hart_id: usize, mode: char) {
    plic::init_hart(hart_id);

    let context = plic::get_context(hart_id, mode);
    for irq in 2..=3 {
        Plic::enable(context, irq);
        Plic::claim(context);
        Plic::complete(context, irq);
    }
    match mode {
        'S' => unsafe {
            sie::set_sext();
        },
        'U' => unsafe {
            uie::set_uext();
        },
        _ => {
            error!("{} mode not supported!", mode);
        }
    }
    TX_FRAMES.store(0, Relaxed);
    RX_FRAMES.store(0, Relaxed);
    info!("xxv_ex: setting up BD rings");
    let mut dma = AXI_DMA.lock();
    // BD ring setup
    dma.rx_bd_create(RX_BD_CNT);
    dma.tx_bd_create(TX_BD_CNT);

    let mut eth = XXV_ETHERNET.lock();
    eth.enter_local_loopback();
    // eth.exit_local_loopback();

    // test single frame
    let mut rx_frame = Box::pin([0u8; XXE_MAX_JUMBO_FRAME_SIZE]);
    let mut tx_frame = Box::pin([0u8; XXE_MAX_JUMBO_FRAME_SIZE]);

    info!(
        "xxv_ex: SrcMacAddr: {:x?}, DestMacAddr: {:x?}",
        SRC_MAC_ADDR, DEST_MAC_ADDR
    );
    fill_frame(tx_frame.as_mut_slice());

    dma.rx_intr_enable();
    dma.rx_submit(&[rx_frame]);
    dma.rx_to_hw();
    dma.tx_intr_enable();
    dma.tx_submit(&[tx_frame]);
    dma.tx_to_hw();

    eth.start();

    info!("waiting for Tx frames");
    while TX_FRAMES.load(Relaxed) == 0 {
        intr_handler(hart_id, context);
    }

    if let Some(bufs) = dma.tx_from_hw() {
        info!("xxv_ex: Tx {} BD from hw", bufs.len());
    } else {
        // panic!("xxv_ex: tx_from_hw failed")
    }

    info!("waiting for Rx frames");
    while RX_FRAMES.load(Relaxed) == 0 {
        intr_handler(hart_id, context);
    }

    if let Some(bufs) = dma.rx_from_hw() {
        info!("xxv_ex: Rx {} BD from hw", bufs.len());
    } else {
        panic!("xxv_ex: rx_from_hw failed")
    }
    eth.stop();

    Plic::disable(context, 2);
    Plic::disable(context, 3);
    match mode {
        'S' => unsafe {
            sie::clear_sext();
        },
        'U' => unsafe {
            uie::clear_uext();
        },
        _ => {
            error!("{} mode not supported!", mode);
        }
    }
}

fn fill_frame(tx_frame: &mut [u8]) {
    // dst addr
    for i in 0..XXE_MAC_ADDR_SIZE {
        tx_frame[i] = DEST_MAC_ADDR[i];
    }

    // src addr
    for i in 0..XXE_MAC_ADDR_SIZE {
        tx_frame[i + XXE_MAC_ADDR_SIZE] = SRC_MAC_ADDR[i];
    }

    // eth type / len
    tx_frame[2 * XXE_MAC_ADDR_SIZE..2 * XXE_MAC_ADDR_SIZE + 2]
        .copy_from_slice(&(PAYLOAD_SIZE as u16).to_be_bytes());

    debug!("xxv_ex: Tx frame header: {:x?}", &tx_frame[..XXE_HDR_SIZE]);

    // fill payload
    let mut payload_size = PAYLOAD_SIZE;
    let mut counter: u16 = 0;
    let mut idx = XXE_HDR_SIZE;

    while payload_size > 0 && counter < 256 {
        tx_frame[idx] = counter as _;
        counter += 1;
        idx += 1;
        payload_size -= 1;
    }

    while payload_size > 0 {
        let high = counter >> 8;
        let low = counter & 0xff;
        tx_frame[idx] = high as _;
        tx_frame[idx + 1] = low as _;
        payload_size -= 1;
    }

    info!("xxv_ex: Tx frame filled");
}

fn verify_frame(tx_frame: &[u8], rx_frame: &[u8]) {}

fn enable_intr() {}

fn intr_handler(hart_id: usize, context: usize) {
    loop {
        let irq = HAS_INTR[hart_id].load(Relaxed);
        if irq == 0 {
            break;
        }

        match irq {
            2 => {
                info!("new dma mm2s intr!");
                AXI_DMA_INTR.lock().tx_intr_handler();
            }
            3 => {
                info!("new dma s2mm intr!");
                AXI_DMA_INTR.lock().rx_intr_handler();
            }
            _ => {
                error!("unsupported ext intr {}!", irq);
            }
        }
        HAS_INTR[hart_id].store(0, Relaxed);
        Plic::complete(context, irq);
    }
}

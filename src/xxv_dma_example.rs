use alloc::boxed::Box;
use core::{convert::TryInto, sync::atomic::Ordering::Relaxed};
use riscv::register::{sie, uie};

use crate::{
    axidma::{AXI_DMA, AXI_DMA_INTR, RX_FRAMES, TX_FRAMES},
    net::{XXE_HDR_SIZE, XXE_MAC_ADDR_SIZE, XXE_MAX_JUMBO_FRAME_SIZE, XXV_ETHERNET},
    plic::{self, Plic},
    HAS_INTR,
};

const RX_BD_CNT: usize = 1024;
const TX_BD_CNT: usize = 1024;

const MAC_ADDR: [u8; XXE_MAC_ADDR_SIZE] = [0x00, 0x00, 0x53, 0x0e, 0x9f, 0xb0];
const SRC_MAC_ADDR: [u8; XXE_MAC_ADDR_SIZE] = [0x00, 0x0A, 0x35, 0x01, 0x02, 0x03];
const DEST_MAC_ADDR: [u8; XXE_MAC_ADDR_SIZE] = [0x00, 0x16, 0x31, 0xf3, 0xc9, 0xad];
const PAYLOAD_SIZE: usize = 8900;

pub fn xxv_dma_example(hart_id: usize, mode: char) {
    info!("xxv_dma_example on hart {}, {} mode", hart_id, mode);

    // setup plic
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

    let mut eth = XXV_ETHERNET.lock();
    eth.reset();

    // test single ethernet frame
    let mut rx_frame = Box::pin([0u8; XXE_MAX_JUMBO_FRAME_SIZE]);
    let mut tx_frame = Box::pin([0u8; XXE_MAX_JUMBO_FRAME_SIZE]);

    eth.enter_local_loopback();
    fill_frame(tx_frame.as_mut_slice(), &MAC_ADDR, &MAC_ADDR);
    // eth.exit_local_loopback();
    // fill_frame(tx_frame.as_mut_slice(), &SRC_MAC_ADDR, &DEST_MAC_ADDR);

    let mut dma = AXI_DMA.lock();
    dma.reset();
    // BD ring setup
    info!("xxv_ex: setting up BD rings");
    dma.rx_bd_create(RX_BD_CNT);
    dma.tx_bd_create(TX_BD_CNT);
    dma.rx_intr_enable();
    dma.rx_submit(&[&rx_frame]);
    dma.rx_to_hw();

    eth.start();
    dma.tx_intr_enable();
    dma.tx_submit(&[&tx_frame]);
    dma.tx_to_hw();

    info!("waiting for Tx frames");
    while TX_FRAMES.load(Relaxed) == 0 {
        intr_handler(hart_id, context);
    }

    if let Some(bufs) = dma.tx_from_hw() {
        info!("xxv_ex: Tx {} BD from hw", bufs.len());
        trace!(
            "xxv_ex: Tx frame addr: 0x{:x}, header: {:x?}",
            bufs[0].as_ptr() as usize,
            &bufs[0][..XXE_HDR_SIZE]
        );
    } else {
        panic!("xxv_ex: tx_from_hw failed")
    }

    info!("waiting for Rx frames");
    while RX_FRAMES.load(Relaxed) == 0 {
        intr_handler(hart_id, context);
    }

    if let Some(bufs) = dma.rx_from_hw() {
        info!("xxv_ex: Rx {} BD from hw", bufs.len());
        trace!(
            "xxv_ex: Rx frame addr: 0x{:x}, header: {:x?}",
            bufs[0].as_ptr() as usize,
            &bufs[0][..XXE_HDR_SIZE]
        );
    } else {
        panic!("xxv_ex: rx_from_hw failed")
    }

    if !verify_frame(tx_frame.as_slice(), rx_frame.as_slice()) {
        error!("frame verification failed!");
    }

    eth.stop();

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

    for irq in 2..=3 {
        Plic::claim(context);
        Plic::complete(context, irq);
        Plic::disable(context, irq);
    }
}

fn fill_frame(tx_frame: &mut [u8], src_mac: &[u8], dest_mac: &[u8]) {
    info!("xxv_ex: src_mac: {:x?}, desc_mac: {:x?}", src_mac, dest_mac);

    // dst addr
    for i in 0..XXE_MAC_ADDR_SIZE {
        tx_frame[i] = dest_mac[i];
    }

    // src addr
    for i in 0..XXE_MAC_ADDR_SIZE {
        tx_frame[i + XXE_MAC_ADDR_SIZE] = src_mac[i];
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
        counter += 1;
        idx += 2;
        payload_size -= 2;
    }

    info!("xxv_ex: Tx frame filled");
}

fn verify_frame(tx_frame: &[u8], rx_frame: &[u8]) -> bool {
    debug!(
        "xxv_ex::verify: Rx frame header: {:x?}",
        &rx_frame[..XXE_HDR_SIZE]
    );
    for i in 0..XXE_HDR_SIZE {
        if tx_frame[i] != rx_frame[i] {
            error!(
                "xxv_ex::verify: Header mismatch at {}: tx: 0x{:x} != rx: 0x{:x}",
                i, tx_frame[i], rx_frame[i]
            );
            return false;
        }
    }

    let mut payload_size = u16::from_be_bytes(
        rx_frame[2 * XXE_MAC_ADDR_SIZE..2 * XXE_MAC_ADDR_SIZE + 2]
            .try_into()
            .unwrap(),
    );

    // TODO: handle 802.1Q Tag

    debug!("xxv_ex::verify: Rx frame len/type: 0x{:x}", payload_size);
    let mut idx = XXE_HDR_SIZE;
    let mut counter: u16 = 0;

    while payload_size > 0 && counter < 256 {
        if rx_frame[idx] != counter as _ {
            error!(
                "xxv_ex::verify: payload mismatch at {}: rx: 0x{:x} != 0x{:x}",
                idx, rx_frame[idx], counter
            );
            return false;
        }
        counter += 1;
        idx += 1;
        payload_size -= 1;
    }

    while payload_size > 0 {
        let high = counter >> 8;
        let low = counter & 0xff;
        if rx_frame[idx] != high as _ || rx_frame[idx + 1] != low as _ {
            error!(
                "xxv_ex::verify: payload mismatch at {}: rx: 0x{:x?} != 0x{:x}",
                idx,
                &rx_frame[idx..idx + 1],
                counter
            );
            return false;
        }
        counter += 1;
        idx += 2;
        payload_size -= 2;
    }

    true
}

fn intr_handler(hart_id: usize, context: usize) {
    let irq_mask = (1 << 2) | (1 << 3);
    loop {
        let irq_masked = HAS_INTR[hart_id].fetch_and(!irq_mask, Relaxed) & irq_mask;
        if irq_masked == 0 {
            break;
        }
        if (irq_masked & (1 << 2)) > 0 {
            info!("new dma mm2s intr!");
            AXI_DMA_INTR.lock().tx_intr_handler();
            Plic::complete(context, 2);
        }
        if (irq_masked & (1 << 3)) > 0 {
            info!("new dma s2mm intr!");
            AXI_DMA_INTR.lock().rx_intr_handler();
            Plic::complete(context, 3);
        }
    }
}

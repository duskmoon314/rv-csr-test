use alloc::sync::Arc;
use lazy_static::lazy_static;
use spin::Mutex;
use xxv_pac::xxv_ethernet;

use crate::{axidma::io_fence, delay};

#[repr(C)]
pub struct EthernetFrameHeader {
    pub dst_mac: [u8; 6],
    pub src_mac: [u8; 6],
    pub eth_type: u16,
}

lazy_static! {
    pub static ref XXV_ETHERNET: Arc<Mutex<Xxvethernet>> =
        Arc::new(Mutex::new(Xxvethernet::default()));
}

pub fn init() {}

/**< XXE_FCS_STRIP_OPTION specifies the Xxv Ethernet device to strip FCS and
 *   PAD from received frames.
 *   This driver sets this option to enabled (set) by default.
 */
pub const XXE_FCS_STRIP_OPTION: usize = 0x00000010;

/**< XXE_FCS_INSERT_OPTION specifies the Xxv Ethernet device to generate the
 *   FCS field and add PAD automatically for outgoing frames.
 *   This driver sets this option to enabled (set) by default.
 */
pub const XXE_FCS_INSERT_OPTION: usize = 0x00000020;

/**< XXE_TRANSMITTER_ENABLE_OPTION specifies the Xxv Ethernet device
 *   transmitter to be enabled.
 *   This driver sets this option to enabled (set) by default.
 */
pub const XXE_TRANSMITTER_ENABLE_OPTION: usize = 0x00000080;

/**< XXE_RECEIVER_ENABLE_OPTION specifies the Xxv Ethernet device receiver to
 *   be enabled.
 *   This driver sets this option to enabled (set) by default.
 */
pub const XXE_RECEIVER_ENABLE_OPTION: usize = 0x00000100;

// XXE_DEFAULT_OPTIONS specify the options set in XXxvEthernet_Reset() and XXxvEthernet_CfgInitialize()
pub const XXE_DEFAULT_OPTIONS: usize = XXE_FCS_INSERT_OPTION
    | XXE_FCS_STRIP_OPTION
    | XXE_TRANSMITTER_ENABLE_OPTION
    | XXE_RECEIVER_ENABLE_OPTION;

/// The next few constants help upper layers determine the size of memory
/// pools used for Ethernet buffers and descriptor lists.

pub const XXE_MAC_ADDR_SIZE: usize = 6; // MAC addresses are 6 bytes
pub const XXE_MTU: usize = 1500; // Max MTU size of an Ethernet frame
pub const XXE_JUMBO_MTU: usize = 8982; // Max MTU size of a jumbo Ethernet frame
pub const XXE_HDR_SIZE: usize = 14; // Size of an Ethernet header
pub const XXE_TRL_SIZE: usize = 4; // Size of an Ethernet trailer (FCS)

pub const XXE_MAX_FRAME_SIZE: usize = XXE_MTU + XXE_HDR_SIZE + XXE_TRL_SIZE;
pub const XXE_MAX_JUMBO_FRAME_SIZE: usize = XXE_JUMBO_MTU + XXE_HDR_SIZE + XXE_TRL_SIZE;

pub const XXE_RX: usize = 1; // Receive direction
pub const XXE_TX: usize = 2; // Transmit direction

pub const RATE_10M: usize = 10;
pub const RATE_100M: usize = 100;
pub const RATE_1G: usize = 1000;
pub const RATE_2G5: usize = 2500;
pub const RATE_10G: usize = 10000;

pub const XXE_LOOPS_TO_COME_OUT_OF_RST: usize = 10000; // Number of loops in the driver

const XXE_RXCFG_DEL_FCS_MASK: u32 = 0x00000002;
const XXE_TXCFG_FCS_MASK: u32 = 0x00000002;
const XXE_TXCFG_TX_MASK: u32 = 0x00000001;
const XXE_RXCFG_RX_MASK: u32 = 0x00000001;
const XXE_ANA_10GKR_MASK: u32 = 0x00000004;
const XXE_SPEED_10_GBPS: usize = 10;
/**< Speed of 10 Gbps */

pub struct Xxvethernet {
    base_address: usize,
    stats: usize,
    is_started: bool,
    is_ready: bool,
    options: usize,
    flag: usize,
}

impl Default for Xxvethernet {
    fn default() -> Self {
        Xxvethernet::new(crate::config::NET_DEVICE_BASE)
    }
}

impl Xxvethernet {
    pub fn new(base_address: usize) -> Self {
        Self {
            base_address,
            stats: 0,
            is_started: false,
            is_ready: true,
            options: XXE_DEFAULT_OPTIONS,
            flag: 0,
        }
    }

    fn hardware(&self) -> &xxv_ethernet::RegisterBlock {
        unsafe { &*(self.base_address as *const _) }
    }

    fn is_started(&self) -> bool {
        self.is_started
    }

    fn is_ready(&self) -> bool {
        self.is_ready
    }

    fn is_tx_err(&self) -> bool {
        self.hardware()
            .txsr
            .read()
            .stat_tx_local_fault()
            .is_enable()
    }

    fn is_rx_err(&self) -> bool {
        let flag = 0xFF0; // [4:11] 位如果存在某位为 1，则出现错误
        self.hardware().rxsr.read().bits() & flag != 0
    }

    fn get_status(&self) -> usize {
        self.hardware().sr.read().bits() as _
    }

    fn is_stats_configured(&self) -> bool {
        self.stats != 0
    }

    pub fn start(&mut self) {
        let mut time_out = 10000000isize;
        // If already started, then there is nothing to do
        if self.is_started() {
            debug!("xxv ethernet is started, there is nothing to do!");
            return;
        }
        debug!("xxvethernet_start");
        // Enable transmitter if not already enabled
        if self.options & XXE_TRANSMITTER_ENABLE_OPTION != 0 {
            debug!("enabling transmitter");
            if self.hardware().txcfg.read().ctl_tx_enable().is_disable() {
                debug!("transmitter not enabled, enabling now");
                self.hardware()
                    .txcfg
                    .modify(|_, w| w.ctl_tx_enable().set_bit());
            }
            debug!("transmitter enabled");
        }

        // Enable receiver
        if self.options & XXE_RECEIVER_ENABLE_OPTION != 0 {
            debug!("enabling receiver");
            if self.hardware().rxcfg.read().ctl_rx_enable().is_disable() {
                debug!("receiver not enabled, enabling now");
                self.hardware()
                    .rxcfg
                    .modify(|_, w| w.ctl_rx_enable().set_bit());
                // RX block lock
                // Do a dummy read because this is a sticky bit
                let _ = self.hardware().rxblsr.read().bits();
                while self
                    .hardware()
                    .rxblsr
                    .read()
                    .stat_rx_block_lock()
                    .is_disable()
                {
                    time_out -= 1;
                    if time_out <= 0 {
                        debug!("ERROR: Block lock is not set");
                        return;
                    }
                }
                debug!("receiver enabled");
            }
        }
        // Mark as started
        self.is_started = true;
        debug!("XXxvEthernet_Start: done");
    }

    pub fn stop(&mut self) {
        // If already stopped, then there is nothing to do
        if !self.is_started() {
            debug!("xxv ethernet is stopped, there is nothing to do!");
            return;
        }
        debug!("xxvethernet_stop");
        debug!("xxvethernet_stop: disabling receiver");
        // Disable the receiver
        self.hardware()
            .rxcfg
            .modify(|_, w| w.ctl_rx_enable().clear_bit());
        // Mark as stopped
        self.is_started = false;
        debug!("xxvethernet_stop: done");
    }

    pub fn reset(&mut self) {
        /*
         * Add delay of 10000 loops to give enough time to the core to come
         * out of reset. Till the time core comes out of reset none of the
         * XxvEthernet registers are accessible including the IS register.
         */
        let mut time_out_loops = XXE_LOOPS_TO_COME_OUT_OF_RST;
        while time_out_loops > 0 {
            time_out_loops -= 1;
        }
        debug!("xxvethernet_reset: {}", time_out_loops);
        self.gt_reset();
        // Stop the device and reset HW
        self.stop();
        self.options = XXE_DEFAULT_OPTIONS;
        // setup HW
        self.init_hw();
    }

    pub fn init_hw(&mut self) {
        let mut reg: usize = 0usize;
        debug!("xxvethernet_init_hw");
        // Disable the receiver
        self.hardware()
            .rxcfg
            .modify(|_, w| w.ctl_rx_enable().disable());

        /*
         * Sync default options with HW but leave receiver and transmitter
         * disabled. They get enabled with XXxvEthernet_Start() if
         * XXE_TRANSMITTER_ENABLE_OPTION and XXE_RECEIVER_ENABLE_OPTION
         * are set
         */
        let options = self.options & !(XXE_TRANSMITTER_ENABLE_OPTION | XXE_RECEIVER_ENABLE_OPTION);

        self.set_options(options);
        self.clear_options(!self.options);
        debug!("xxvethernet_init_hw: done");
    }

    pub fn set_options(&mut self, options: usize) {
        // Be sure device has been stopped
        if self.is_started() {
            debug!("device is not stopped");
            return;
        }
        debug!("xxvethernet_set_options");
        // Set options word to its new value
        self.options |= options;

        // Many of these options will change the RCFG or TCFG registers.
        // To reduce the amount of IO to the device, group these options here
        // and change them all at once.
        /* Get current register contents */
        let reg_rcfg = self.hardware().rxcfg.read().bits(); // Reflects original contents of RCFG
        let reg_tc = self.hardware().txcfg.read().bits(); // Reflects original contents of TC
        let mut reg_new_rcfg = reg_rcfg; // Reflects new contents of RCFG
        let mut reg_new_tc = reg_tc; // Reflects new contents of TC
        debug!(
            "current control regs: RCFG: {:#x}; TC: {:#x}",
            reg_rcfg, reg_tc
        );
        debug!(
            "options: {:#x}; default options: {:#x}",
            options, XXE_DEFAULT_OPTIONS
        );
        // Turn on FCS stripping on receive packets
        if options & XXE_FCS_STRIP_OPTION != 0 {
            debug!("set_options: enabling fcs stripping");
            reg_new_rcfg |= XXE_RXCFG_DEL_FCS_MASK;
        }
        // Turn on FCS insertion on transmit packets
        if options & XXE_FCS_INSERT_OPTION != 0 {
            debug!("set_options: enabling fcs insertion");
            reg_new_tc |= XXE_TXCFG_FCS_MASK;
        }
        // Enable transmitter
        if options & XXE_TRANSMITTER_ENABLE_OPTION != 0 {
            reg_new_tc |= XXE_TXCFG_TX_MASK;
        }
        // Enable receiver
        if options & XXE_RECEIVER_ENABLE_OPTION != 0 {
            reg_new_rcfg |= XXE_RXCFG_RX_MASK;
        }
        // Change the TC or RCFG registers if they need to be modified
        if reg_tc != reg_new_tc {
            debug!("set_options: writing tc: {:#x}", reg_new_tc);
            self.hardware()
                .txcfg
                .write(|w| unsafe { w.bits(reg_new_tc) });
        }
        if reg_rcfg != reg_new_rcfg {
            debug!("set_options: writing rcfg: {:#x}", reg_new_rcfg);
            self.hardware()
                .rxcfg
                .write(|w| unsafe { w.bits(reg_new_rcfg) });
        }
        debug!("set_options: returning success");
    }

    pub fn clear_options(&mut self, options: usize) {
        debug!("xxvethernet_clear_options: {:#x}", options);
        // Be sure device has been stopped
        if self.is_started() {
            debug!("device is not stopped");
            return;
        }
        // Set options word to its new value.
        self.options &= !options;

        // Many of these options will change the RCFG or TC registers.
        // Group these options here and change them all at once. What we are
        // trying to accomplish is to reduce the amount of IO to the device
        let reg_rcfg = self.hardware().rxcfg.read().bits(); // Reflects original contents of RCFG
        let reg_tc = self.hardware().txcfg.read().bits(); // Reflects original contents of TC
        let mut reg_new_rcfg = reg_rcfg; // Reflects new contents of RCFG
        let mut reg_new_tc = reg_tc; // Reflects new contents of TC

        // Turn off FCS stripping on receive packets
        if options & XXE_FCS_STRIP_OPTION != 0 {
            debug!("xxvethernet_clear_options: disabling fcs strip");
            reg_new_rcfg &= !XXE_RXCFG_DEL_FCS_MASK;
        }
        // Turn off FCS insertion on transmit packets
        if options & XXE_FCS_INSERT_OPTION != 0 {
            debug!("xxvethernet_clear_options: disabling fcs insert");
            reg_new_tc &= !XXE_TXCFG_FCS_MASK;
        }
        // Disable transmitter
        if options & XXE_TRANSMITTER_ENABLE_OPTION != 0 {
            debug!("xxvethernet_clear_options: disabling transmitter");
            reg_new_tc &= !XXE_TXCFG_TX_MASK;
        }
        // Disable receiver
        if options & XXE_RECEIVER_ENABLE_OPTION != 0 {
            debug!("xxvethernet_clear_options: disabling receiver");
            reg_new_rcfg &= !XXE_RXCFG_RX_MASK;
        }
        // Change the TC and RCFG registers if they need to be modified
        if reg_tc != reg_new_tc {
            debug!("clear_options: setting TC: {:#x}", reg_new_tc);
            self.hardware()
                .txcfg
                .write(|w| unsafe { w.bits(reg_new_tc) });
        }
        if reg_rcfg != reg_new_rcfg {
            debug!("clear_options: setting RCFG: {:#x}", reg_new_rcfg);
            self.hardware()
                .rxcfg
                .write(|w| unsafe { w.bits(reg_new_rcfg) });
        }
        debug!("clear_options: returning success");
    }

    pub fn get_options(&mut self) -> usize {
        self.options
    }

    pub fn get_autonegspeed(&mut self) -> usize {
        debug!("xxvethernet_get_operating_speed");
        if self
            .hardware()
            .anasr
            .read()
            .stat_an_lp_ability_10gbase_kr()
            .is_enable()
        {
            debug!("xxvethernet_get_operating_speed: returning 1000");
            XXE_SPEED_10_GBPS
        } else {
            0
        }
    }

    pub fn set_autonegspeed(&mut self) {
        debug!("xxvethernet_set_operating_speed");
        self.hardware()
            .anacr
            .modify(|_, w| w.ctl_an_ability_10gbase_kr().set_bit());
        debug!("xxvethernet_set_operating_speed: done");
    }

    #[allow(unused)]
    pub fn enter_local_loopback(&mut self) {
        self.hardware()
            .mode
            .modify(|_, w| w.ctl_local_loopback().enable());
        self.gt_reset();
    }

    #[allow(unused)]
    pub fn exit_local_loopback(&mut self) {
        self.hardware()
            .mode
            .modify(|_, w| w.ctl_local_loopback().disable());
        self.gt_reset();
    }

    fn gt_reset(&self) {
        self.hardware()
            .grr
            .modify(|_, w| w.ctl_gt_reset_all().enable());
        delay(1);
        self.hardware()
            .grr
            .modify(|_, w| w.ctl_gt_reset_all().disable());
        let mut poll_timeout = 10;
        while poll_timeout > 0 {
            let gw = &self.hardware().gtwizsr;
            if gw.read().gtwiz_reset_tx_done().is_disable()
                || gw.read().gtwiz_reset_rx_done().is_disable()
            {
                poll_timeout -= 1;
                delay(1);
            } else {
                break;
            }
        }
        if poll_timeout == 0 {
            error!("xxv::gt_reset: XXV MAC GT reset not complete! Cross-check the MAC ref clock configuration");
        } else {
            debug!("xxv::gt_reset: XXV MAC GT reset completed");
        }

        poll_timeout = 10;
        while poll_timeout > 0 {
            let st = &self.hardware().rxblsr;
            if st.read().stat_rx_block_lock().is_disable() {
                poll_timeout -= 1;
                delay(1);
            } else {
                break;
            }
        }
        if poll_timeout == 0 {
            error!("xxv::gt_reset: XXV MAC RX block lock not complete! Cross-check the MAC ref clock configuration");
        } else {
            debug!("xxv::gt_reset: XXV MAC RX block lock completed");
        }
    }
}

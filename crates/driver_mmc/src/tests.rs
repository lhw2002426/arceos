//extern crate bcm2837;
extern crate alloc;
use log::{info, LevelFilter};
use env_logger::Builder;
use crate::bcm2835_sdhci::*;
use bcm2837::emmc::*;
use alloc::sync::Arc;
use spinlock::{BaseSpinLock,BaseSpinLockGuard,SpinRaw,SpinNoIrq as Mutex};
use driver_common::{BaseDriverOps, DevError, DevResult, DeviceType};
use driver_block::BlockDriverOps;
fn setup_logging() {
    Builder::new()
        .filter_level(LevelFilter::Info)
        .init();
}

#[test]
fn test_mmc() {
    setup_logging();
    debug!("Initializing EmmcCtl...");
    info!("lhw test");
    let mut ctrl = EmmcCtl::new();
    info!("after new");
    if ctrl.init() == 0 {
        //demo(&mut ctrl);
        //demo_write(&mut ctrl);
        info!("ctrl successfilly init");
        let mut driver = SDHCIDriver(Mutex::new(ctrl));
        info!("BCM2835 sdhci: successfully initialized");
        let empty_slice: &mut [u8] = &mut [];
        let _ = driver.read_block(0, empty_slice);
    } else {
        info!("ctrl fail to init");
        warn!("BCM2835 sdhci: init failed");
    }
}
//! sd card driver for raspi4
extern crate alloc;
use crate::BlockDriverOps;
use alloc::sync::Arc;
use bcm2835_sdhci::Bcm2835SDhci::{EmmcCtl, BLOCK_SIZE};
use core::slice;
use driver_common::{BaseDriverOps, DevError, DevResult, DeviceType};
use spinlock::SpinNoIrq as Mutex;

///sdhci driver
pub struct SDHCIDriver(pub Mutex<EmmcCtl>);

impl SDHCIDriver {
    ///sd driver new
    pub fn new() -> SDHCIDriver {
        let mut ctrl = EmmcCtl::new();
        if ctrl.init() == 0 {
            info!("BCM2835 sdhci: successfully initialized");
        } else {
            warn!("BCM2835 sdhci: init failed");
        }
        SDHCIDriver(Mutex::new(ctrl))
    }
}

impl BaseDriverOps for SDHCIDriver {
    fn device_type(&self) -> DeviceType {
        DeviceType::Block
    }

    fn device_name(&self) -> &str {
        "bcm2835_sdhci"
    }
}

impl BlockDriverOps for SDHCIDriver {
    fn read_block(&mut self, block_id: u64, buf: &mut [u8]) -> DevResult {
        if buf.len() < BLOCK_SIZE {
            return Err(DevError::Io);
        }
        let buf = unsafe { slice::from_raw_parts_mut(buf.as_ptr() as *mut u32, BLOCK_SIZE / 4) };
        let res = self.0.lock().read_block(block_id as u32, 1, buf);
        match res {
            Ok(()) => Ok(()),
            Err(e) => Err(DevError::Io),
        }
    }

    fn write_block(&mut self, block_id: u64, buf: &[u8]) -> DevResult {
        if buf.len() < BLOCK_SIZE {
            return Err(DevError::Io);
        }
        let buf = unsafe { slice::from_raw_parts(buf.as_ptr() as *mut u32, BLOCK_SIZE / 4) };
        let res = self.0.lock().write_block(block_id as u32, 1, buf);
        match res {
            Ok(()) => Ok(()),
            Err(e) => Err(DevError::Io),
        }
    }
    fn flush(&mut self) -> DevResult {
        Ok(())
    }
    #[inline]
    fn num_blocks(&self) -> u64 {
        4194304
    }

    #[inline]
    fn block_size(&self) -> usize {
        self.0.lock().get_block_size()
    }
}

///sd init
pub fn init() {
    info!("Initializing EmmcCtl...");
    let mut ctrl = EmmcCtl::new();
    if ctrl.init() == 0 {
        //demo(&mut ctrl);
        //demo_write(&mut ctrl);
        let driver = Arc::new(SDHCIDriver(Mutex::new(ctrl)));
        //rigister in rCore dosnt use
        /* DRIVERS.write().push(driver.clone());
        IRQ_MANAGER.write().register_all(driver.clone());
        BLK_DRIVERS.write().push(driver);*/
        info!("BCM2835 sdhci: successfully initialized");
    } else {
        warn!("BCM2835 sdhci: init failed");
    }
}

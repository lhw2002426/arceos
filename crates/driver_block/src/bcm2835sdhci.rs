//! sd card driver for raspi4
extern crate alloc;
use crate::BlockDriverOps;
use bcm2835_sdhci::Bcm2835SDhci::{EmmcCtl, BLOCK_SIZE};
use bcm2835_sdhci::SDHCIError;
use driver_common::{BaseDriverOps, DevError, DevResult, DeviceType};

///sdhci driver
pub struct SDHCIDriver(pub EmmcCtl);

impl SDHCIDriver {
    ///sd driver new
    pub fn new() -> SDHCIDriver {
        let mut ctrl = EmmcCtl::new();
        if ctrl.init() == 0 {
            info!("BCM2835 sdhci: successfully initialized");
        } else {
            warn!("BCM2835 sdhci: init failed");
        }
        SDHCIDriver(ctrl)
    }
}

fn deal_sdhci_err(err: SDHCIError) -> DevError {
    match err {
        SDHCIError::Io => DevError::Io,
        SDHCIError::AlreadyExists => DevError::AlreadyExists,
        SDHCIError::Again => DevError::Again,
        SDHCIError::BadState => DevError::BadState,
        SDHCIError::InvalidParam => DevError::InvalidParam,
        SDHCIError::NoMemory => DevError::NoMemory,
        SDHCIError::ResourceBusy => DevError::ResourceBusy,
        SDHCIError::Unsupported => DevError::Unsupported,
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
            return Err(DevError::InvalidParam);
        }
        let (prefix, aligned_buf, suffix) = unsafe { buf.align_to_mut::<u32>() };
        if !prefix.is_empty() || !suffix.is_empty() {
            return Err(DevError::InvalidParam);
        }
        self.0
            .read_block(block_id as u32, 1, aligned_buf)
            .map_err(deal_sdhci_err)
    }

    fn write_block(&mut self, block_id: u64, buf: &[u8]) -> DevResult {
        if buf.len() < BLOCK_SIZE {
            return Err(DevError::Io);
        }
        //let buf = unsafe { slice::from_raw_parts(buf.as_ptr() as *mut u32, BLOCK_SIZE / 4) };
        let (prefix, aligned_buf, suffix) = unsafe { buf.align_to::<u32>() };

        if !prefix.is_empty() || !suffix.is_empty() {
            return Err(DevError::InvalidParam);
        }
        self.0
            .write_block(block_id as u32, 1, aligned_buf)
            .map_err(deal_sdhci_err)
    }
    fn flush(&mut self) -> DevResult {
        Ok(())
    }
    #[inline]
    fn num_blocks(&self) -> u64 {
        self.0.get_block_num()
    }

    #[inline]
    fn block_size(&self) -> usize {
        self.0.get_block_size()
    }
}

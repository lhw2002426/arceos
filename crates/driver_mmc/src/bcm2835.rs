//bcm2835.rs
use driver_common::{BaseDriverOps, DevError, DevResult, DeviceType};
use driver_block::BlockDriverOps;
use crate::constants::*;

#[derive(Default)]
pub struct SDHCIdevice{
    pub flag:i32,
    size:usize,
}
impl SDHCIdevice{
    pub fn new() -> SDHCIdevice{
        SDHCIdevice{
            flag:0,
            size:0,
        }
    }
}

impl BaseDriverOps for SDHCIdevice {
    fn device_type(&self) -> DeviceType {
        DeviceType::Block
    }

    fn device_name(&self) -> &str {
        "SDHCIdevice"
    }
}

impl BlockDriverOps for SDHCIdevice {
    #[inline]
    fn num_blocks(&self) -> u64 {
        (self.size / BLOCK_SIZE) as u64
    }

    #[inline]
    fn block_size(&self) -> usize {
        BLOCK_SIZE
    }

    fn read_block(&mut self, block_id: u64, buf: &mut [u8]) -> DevResult {
        let offset = block_id as usize * BLOCK_SIZE;
        if offset + buf.len() > self.size {
            return Err(DevError::Io);
        }
        if buf.len() % BLOCK_SIZE != 0 {
            return Err(DevError::InvalidParam);
        }
        self.0.lock().read_block(block_id as u32, 1, buf);
        Ok(())
    }

    fn write_block(&mut self, block_id: u64, buf: &[u8]) -> DevResult {
        let offset = block_id as usize * BLOCK_SIZE;
        if offset + buf.len() > self.size {
            return Err(DevError::Io);
        }
        if buf.len() % BLOCK_SIZE != 0 {
            return Err(DevError::InvalidParam);
        }
        self.0.lock().write_block(block_id as u32, 1, buf);
        Ok(())
    }

    fn flush(&mut self) -> DevResult {
        Ok(())
    }
}
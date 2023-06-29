extern crate alloc;

use alloc::sync::{Arc,Weak};
use std::io::{Error, Result};
use crate::constants::*;
use driver_common::{BaseDriverOps, DevError, DevResult, DeviceType};
use spinlock::{BaseSpinLock,BaseSpinLockGuard,SpinRaw,SpinNoIrq,SpinNoIrqGuard};
use axsync::Mutex;//todo axsync is in module
#[derive(Default)]
pub struct Bcm2835Host{
    pub max_blk_size:u32,
    pub max_blk_count:u32,
    pub max_req_size:u32,
    pub max_delay:u32,
    pub mrq: Option<Arc<SpinNoIrq<MmcRequest>>>,
    pub car_busy:bool,
    pub spinlock:SpinRaw<u32>,
    pub use_dma:bool,
    pub cmd:Option<Arc<MmcCommand>>,
}

impl Bcm2835Host{
    pub fn new()->Bcm2835Host{
        Bcm2835Host { 
            ..Bcm2835Host::default()
         }
    }
}

#[derive(Default)]
pub struct MmcCommand {
    pub opcode: u32,
    pub arg: i32,
    pub resp: [u32; 4],
    pub flags: u32,
    pub retries: u32,
    pub error: i32,
    pub busy_timeout:u32,
    //mrq: Option<&MmcRequest>,
    pub data: Option<Arc<SpinNoIrq<MmcData>>>,
}

impl MmcCommand {
    pub fn new() -> MmcCommand {
        MmcCommand {
            ..MmcCommand::default()
        }
    }
    pub fn clone(&self) -> MmcCommand {
        MmcCommand {
            opcode: self.opcode,
            arg: self.arg,
            resp: self.resp,
            flags: self.flags,
            retries: self.retries,
            error: self.error,
            //mrq: self.mrq,
            data: self.data.clone(),
            busy_timeout:self.busy_timeout,
        }
    }
}

#[derive(Default)]
pub struct MmcData {
    timeout: u32,
    blksz: u32,
    blk_addr: u32,
    error: i32,
    flags: u32,
    blocks: u32,
    //mrq: Option<Weak<MmcRequest>>,
    //stop: Option<Weak<MmcCommand>>,
}

impl MmcData {
    fn clone(&self) -> MmcData {
        MmcData {
            timeout: self.timeout,
            blksz: self.blksz,
            blk_addr: self.blk_addr,
            error: self.error,
            flags: self.flags,
            blocks: self.blocks,
            //mrq: self.mrq,
            //stop: self.stop,
        }
    }
}

#[derive(Default)]
pub struct MmcRequest {
    pub cmd: Option<Arc<SpinNoIrq<MmcCommand>>>,
    pub data: Option<Arc<SpinNoIrq<MmcData>>>,
    pub stop: Option<Arc<SpinNoIrq<MmcCommand>>>,
}

impl MmcRequest {
    pub fn new() -> MmcRequest {
        MmcRequest {
            cmd: Some(Arc::new(SpinNoIrq::new(MmcCommand::new()))),
            data: Some(Arc::new(SpinNoIrq::new(MmcData::default()))),
            stop: Some(Arc::new(SpinNoIrq::new(MmcCommand::new()))),
        }
    }

    pub fn clone(&self) -> MmcRequest {
        MmcRequest {
            cmd: self.cmd.clone(),
            data: self.data.clone(),
            stop: self.stop.clone(),
        }
    }
    
    pub fn prepare_mrq(&mut self,dev_addr:u32,blocks:u32,blksz:u32,write:bool) {
        if let Some(cmd) = self.cmd.as_mut()
        {
            if let mut cmd_ref = cmd.lock(){
                if blocks > 1 {
                    cmd_ref.opcode = if write {
                        MMC_WRITE_MULTIPLE_BLOCK
                    } else {
                        MMC_READ_MULTIPLE_BLOCK
                    };
                } else {
                    cmd_ref.opcode = if write {
                        MMC_WRITE_BLOCK
                    } else {
                        MMC_READ_SINGLE_BLOCK
                    };
                }
                cmd_ref.arg = dev_addr as i32;
                /*if !mmc_card_blockaddr(test.card) { //todo
                    mrq.cmd.arg <<= 9;
                }*/
                cmd_ref.flags = MMC_RSP_R1 | MMC_CMD_ADTC;
            }
        }
        if blocks == 1 {
            self.stop = None;
        } else {
            if let Some(stop) = self.stop.as_mut(){
                
                if let mut stop_ref = stop.lock(){
                    stop_ref.opcode = MMC_STOP_TRANSMISSION;
                    stop_ref.arg = 0;
                    stop_ref.flags = MMC_RSP_R1B | MMC_CMD_AC;
                    stop_ref.resp = [0; 4];
                }
            }
        }

        if let Some(data) = self.data.as_mut(){
            if let mut data_ref = data.lock(){
                data_ref.blksz = blksz;
                data_ref.blocks = blocks;
                data_ref.flags = if write {
                    MMC_DATA_WRITE
                } else {
                    MMC_DATA_READ
                };
            }
        }
        
        //self.data.sg = sg;
        //self.data.sg_len = sg_len;
        
        //mmc_test_prepare_sbc(test, mrq, blocks);//todo

	    //mmc_set_data_timeout(mrq->data, test->card);//todo
    }
    pub fn mmc_mrq_prep(&mut self,max_blk_size:u32,max_blk_count:u32,max_req_size:u32) -> DevResult{
        if let Some(cmd) = self.cmd.as_mut(){
            if let mut cmd_ref = cmd.lock(){
                cmd_ref.error = 0;
                //cmd_ref.mrq = Some(self);
                if let Some(data) = self.data.as_ref() {
                    cmd_ref.data = Some(data.clone());
                }
            }
        }
        
        /*if let Some(sbc) = &mut mrq.sbc {//todo
            sbc.error = 0;
            sbc.mrq = Some(&mut mrq);
        }*/
        if let Some(data) = self.data.as_mut(){
            if let data_ref = data.lock(){
                if data_ref.blksz > max_blk_size
                    || data_ref.blocks > max_blk_count
                    || data_ref.blocks * data_ref.blksz > max_req_size
                {
                    return Err(DevError::InvalidParam);
                }
            
            /*let mut sz = 0;
            for sg in mrq.data.sg.iter().take(mrq.data.sg_len) {
                sz += sg.length;
            }
        
            if sz != data.blocks * data.blksz {
                return -EINVAL;
            }*/
                if let Some(stop) = self.stop.as_mut(){
                    if let mut stop_ref = stop.lock(){
                    //data_ref.stop = Some(); todo
                    stop_ref.error = 0;
                    //stop_ref.mrq = Some(self);
                    }
                }
            }
        }
        Ok(())
    }
}
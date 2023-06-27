use std::io::{Error, Result};
use crate::constants::*;
#[derive(Default)]
pub struct Bcm2835Host{
    pub max_blk_size:u32,
    pub max_blk_count:u32,
    pub max_req_size:u32,
}

impl Bcm2835Host{
    pub fn new()->Bcm2835Host{
        Bcm2835Host { 
            ..Bcm2835Host::default()
         }
    }
}

#[derive(Default)]
pub struct MmcCommand{
    opcode:u32,
    arg:u32,
    resp:[u32; 4],
    flags:u32,
    retries:u32,
    error:i32,
    mrq: Option<&'static mut MmcRequest>,
    data: Option<*mut MmcData>,
}

impl MmcCommand{
    pub fn new() -> MmcCommand{
        MmcCommand {
            ..MmcCommand::default()
         }
    }
}

#[derive(Default)]
pub struct MmcData{
    timeout:u32,
    blksz:u32,
    blk_addr:u32,
    error:i32,
    flags:u32,
    blocks:u32,
    mrq: Option<&'static mut MmcRequest>,
    stop: Option<*mut MmcCommand>,
}

#[derive(Default)]
pub struct MmcRequest {
    cmd: Option<MmcCommand>,
    data: Option<MmcData>,
    stop: Option<MmcCommand>,
}

impl MmcRequest {
    pub fn new()->MmcRequest{
        MmcRequest { 
            cmd: Some(MmcCommand::new()),
            data: Some(MmcData::default()),
            stop: Some(MmcCommand::new()),
        }
    }
    pub fn prepare_mrq(&mut self,dev_addr:u32,blocks:u32,blksz:u32,write:bool) {
        if let Some(cmd) = self.cmd.as_mut()
        {
            if blocks > 1 {
                cmd.opcode = if write {
                    MMC_WRITE_MULTIPLE_BLOCK
                } else {
                    MMC_READ_MULTIPLE_BLOCK
                };
            } else {
                cmd.opcode = if write {
                    MMC_WRITE_BLOCK
                } else {
                    MMC_READ_SINGLE_BLOCK
                };
            }
            
            cmd.arg = dev_addr;
            /*if !mmc_card_blockaddr(test.card) { //todo
                mrq.cmd.arg <<= 9;
            }*/
            
            cmd.flags = MMC_RSP_R1 | MMC_CMD_ADTC;
        }
        
        if blocks == 1 {
            self.stop = None;
        } else {
            self.stop = Some(MmcCommand {
                opcode: MMC_STOP_TRANSMISSION,
                arg: 0,
                flags: MMC_RSP_R1B | MMC_CMD_AC,
                resp: [0; 4],
                ..Default::default()
            });
        }
        if let Some(data) = self.data.as_mut() {
            data.blksz = blksz;
            data.blocks = blocks;
            data.flags = if write {
                                MMC_DATA_WRITE
                            } else {
                                MMC_DATA_READ
                            };
        }
        
        //self.data.sg = sg;
        //self.data.sg_len = sg_len;
        
        //mmc_test_prepare_sbc(test, mrq, blocks);//todo

	    //mmc_set_data_timeout(mrq->data, test->card);//todo
    }
    pub fn mmc_mrq_prep(&mut self,max_blk_size:u32,max_blk_count:u32,max_req_size:u32) -> Result<()>{
        if let Some(cmd) = self.cmd.as_mut() {
            cmd.error = 0;
            cmd.mrq = Some(self);
            if let Some(data) = self.data.as_mut(){
                cmd.data = Some(&mut *data);
            }
        }
        
        /*if let Some(sbc) = &mut mrq.sbc {//todo
            sbc.error = 0;
            sbc.mrq = Some(&mut mrq);
        }*/
        
        if let Some(data) = self.data.as_mut() {
            if data.blksz > max_blk_size
                || data.blocks > max_blk_count
                || data.blocks * data.blksz > max_req_size
            {
                return Err(Error::InvalidData);
            }
        
            /*let mut sz = 0;
            for sg in mrq.data.sg.iter().take(mrq.data.sg_len) {
                sz += sg.length;
            }
        
            if sz != data.blocks * data.blksz {
                return -EINVAL;
            }*/
        
            if let Some(stop) = self.stop.as_mut() {
                data.stop = Some(&mut *stop);
                stop.error = 0;
                stop.mrq = Some(self);
            }
        }
        Ok(())
    }
}


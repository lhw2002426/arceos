//bcm2835.rs
use driver_common::{BaseDriverOps, DevError, DevResult, DeviceType};
use driver_block::BlockDriverOps;
use crate::constants::*;
use crate::structs::*;

impl Bcm2835Host {
    fn wait_ongoing_tfr_cmd(&mut self){
        
    }
    fn mmc_mrq_pr_debug (&mut self, mrq: &mut MmcRequest){
        //todo print mrq info
    }
    fn bcm2835_mmc_request(&mut self, mrq: &mut MmcRequest) -> DevResult{
        let host: &mut Bcm2835Host = mmc_priv(mmc);

        let flags: unsigned long;
        spin_lock_irqsave(&host.lock, &flags);

        WARN_ON(host.mrq.is_some());

        host.mrq = Some(mrq);

        if mrq.sbc.is_some() && !(host.flags & SDHCI_AUTO_CMD23) {
            bcm2835_mmc_send_command(host, mrq.sbc.unwrap());
        } else {
            bcm2835_mmc_send_command(host, mrq.cmd);
        }

        spin_unlock_irqrestore(&host.lock, flags);

        if !(mrq.sbc.is_some() && !(host.flags & SDHCI_AUTO_CMD23)) && mrq.cmd.data.is_some() && host.use_dma {
            // DMA transfer starts now, PIO starts after interrupt
            bcm2835_mmc_transfer_dma(host);
        }
    }
    fn mmc_start_request(&mut self, mrq: &mut MmcRequest) -> DevResult{
        /*
        init_completion(&mrq->cmd_completion);

        mmc_retune_hold(host);//todo

        if (mmc_card_removed(host->card))
            return -ENOMEDIUM;
         */
        self.mmc_mrq_pr_debug(mrq);
        mrq.mmc_mrq_prep(self.max_blk_size,self.max_blk_count,self.max_req_size);
        //let err = mmc_retune(host); todo

        if sdio_is_io_busy(mrq.cmd.opcode, mrq.cmd.arg) && host.ops.card_busy.is_some() {
            let mut tries = 500; // Wait approx 500ms at maximum

            while host.ops.card_busy.unwrap()(host) && tries > 0 {
                mmc_delay(1);
                tries -= 1;
            }

            if tries == 0 {
                mrq.cmd.error = -EBUSY;
                mmc_request_done(host, mrq);
                return;
            }
        }

        /*if mrq.cap_cmd_during_tfr {
            host.ongoing_mrq = Some(mrq);
            reinit_completion(&mrq.cmd_completion);
        }

        trace_mmc_request_start(host, mrq);

        if host.cqe_on {
            host.cqe_ops.as_ref().unwrap().cqe_off(host);
        }*/

        let res = self.bcm2835_mmc_request(&mut mrq);
        match res{
            Ok(data)=>{
                return Ok(());
            }
            Err(err)=>{
                //mrq->error = err;
                return Err(DevError::Io);
            }
        }

    }
    pub fn request_with_wait(&mut self, mrq: &mut MmcRequest) -> DevResult {
        self.wait_ongoing_tfr_cmd();//todo
        //mrq->done = mmc_wait_done;//todo
        let res = self.mmc_start_request(&mut mrq);
        match res{
            Ok(data)=>{
                return Ok(());
            }
            Err(err)=>{
                //mrq->error = err;
                return Err(DevError::Io);
            }
        }
    }
}

#[derive(Default)]
pub struct SDHCIdevice{
    pub flag:i32,
    size:usize,
    pub host:Bcm2835Host,
}
impl SDHCIdevice{
    pub fn new() -> SDHCIdevice{
        SDHCIdevice{
            flag:0,
            size:512,
            host:Bcm2835Host::new(),
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

    fn read_block(&mut self, block_id: u64, buf: &mut [u8]) -> DevResult {//todo the relationship between block_id and dev_addr,how to convert?
        let offset = block_id as usize * BLOCK_SIZE;
        if offset + buf.len() > self.size {
            return Err(DevError::Io);
        }
        if buf.len() % BLOCK_SIZE != 0 {
            return Err(DevError::InvalidParam);
        }
        let mut mrq = MmcRequest::new();
        mrq.prepare_mrq(0, 0, 0, false);//todo should params include data?
        self.host.request_with_wait(&mut mrq);
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
        let mut mrq = MmcRequest::new();
        mrq.prepare_mrq(0, 0, 0, true);
        self.host.request_with_wait(&mut mrq);
        Ok(())
    }

    fn flush(&mut self) -> DevResult {
        Ok(())
    }
}
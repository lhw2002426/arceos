//bcm2835.rs
extern crate alloc;

use driver_common::{BaseDriverOps, DevError, DevResult, DeviceType};
use driver_block::BlockDriverOps;
use alloc::sync::Arc;
use crate::constants::*;
use crate::structs::*;
use std::thread;
use std::time::Duration;
use axsync::Mutex;
use spinlock::{BaseSpinLock,BaseSpinLockGuard,SpinRaw,SpinNoIrq};

const int mmc_debug = 0;

impl Bcm2835Host {
    fn wait_ongoing_tfr_cmd(&mut self){
        //todo check busy flag and wait
    }
    fn mmc_mrq_pr_debug (&mut self, mrq: & MmcRequest){
        //todo print mrq info

    }
    fn bcm2835_readl(addr:i32)->u32{
        return 0;
    }
    fn bcm2835_mmc_readl (&mut self, reg:u32)->u32{
        lockdep_assert_held_once(&host.lock);
        return readl(self.ioaddr + reg);
    }
    fn bcm2835_mmc_writel (&mut self, value: i32,reg: u32,form: i32){
        let delay: u32;
        lockdep_assert_held_once(&host.lock);
        writel(val, host.ioaddr + reg);
        //udelay(BCM2835_SDHCI_WRITE_DELAY(max(host.clock, MIN_FREQ)));
        let duration = Duration::from_micros(BCM2835_SDHCI_WRITE_DELAY(max(host.clock, MIN_FREQ))); 
        thread::sleep(duration);

        delay = ((mmc_debug >> 16) & 0xf) << ((mmc_debug >> 20) & 0xf);
        if delay != 0 && !((1 << from) & mmc_debug2 != 0) {
            //udelay(delay);
            let duration = Duration::from_micros(delay); 
            thread::sleep(duration);
        }
    }
    fn bcm2835_mmc_writew (&mut self, value: i32,reg: u32){
        let oldval = if reg == SDHCI_COMMAND {
            host.shadow
        } else {
            bcm2835_mmc_readl(host, reg & !3)
        };
        let word_num = (reg >> 1) & 1;
        let word_shift = word_num * 16;
        let mask = 0xffff << word_shift;
        let newval = (oldval & !mask) | (val as u32) << word_shift;
    
        if reg == SDHCI_TRANSFER_MODE {
            host.shadow = newval;
        } else {
            bcm2835_mmc_writel(host, newval, reg & !3, 0);
        }
    }
    fn bcm2835_mmc_prepare_data (&mut self, mrq: & MmcCommand){
        //todo prepare data
    }
    fn bcm2835_mmc_set_transfer_mode (&mut self, mrq: & MmcCommand){
        //todo set trans mode
    }
    fn bcm2835_mmc_dumpregs(&self){
        //todo print reg info
    }
    fn bcm2835_mmc_send_command(&mut self,cmd:& MmcCommand) -> DevResult{
        let mut flags: u32;
        let mut mask: u32;
        let mut timeout: u32;

        //WARN_ON(host->cmd);

        // Wait max 10 ms
        timeout = 1000;

        mask = SDHCI_CMD_INHIBIT;
        if cmd.data.is_some() || (cmd.flags & MMC_RSP_BUSY) != 0 {
            mask |= SDHCI_DATA_INHIBIT;
        }

        // We shouldn't wait for data inhibit for stop commands, even
        // though they might use busy signaling
        if let Some(mrq) = self.mrq.as_mut(){
            if let  mut mrq_ref = mrq.lock(){
                if let Some(data) = mrq_ref.data.as_mut(){
                    if let mut data_ref = data.lock(){
                        /*if cmd as *const _ == data.stop as *const _ { todo
                            
                        }*/
                        mask &= !SDHCI_DATA_INHIBIT;
                    }
                }
            }
        }
        while self.bcm2835_mmc_readl(SDHCI_PRESENT_STATE) & mask != 0 {
            if timeout == 0 {
                /*pr_err!(
                    "{}: Controller never released inhibit bit(s).\n",
                    mmc_hostname(host.mmc)
                );*///todo how to print error
                self.bcm2835_mmc_dumpregs();
                //cmd.error = -EIO;
                //tasklet_schedule(&mut host.finish_tasklet);
                return Err(DevError::Io);
            }
            timeout -= 1;
            let duration = Duration::from_micros(10); 
            thread::sleep(duration);
            //udelay(10);
        }

        if (1000 - timeout) / 100 > 1 && (1000 - timeout) / 100 > self.max_delay {
            self.max_delay = (1000 - timeout) / 100;
            //pr_warn!("Warning: MMC controller hung for {} ms\n", host.max_delay);
        }

        //timeout = jiffies;//todo
        if cmd.data.is_none() && cmd.busy_timeout > 9000 {
            timeout += (cmd.busy_timeout + 999) / 1000 * HZ + HZ;
        } else {
            timeout += 10 * HZ;
        }
        //mod_timer(&mut host.timer, timeout); //todo system timeout?

        self.cmd = Some(Arc::clone(&Arc::new((*cmd).clone())));//use *cmd will get the value of cmd and crate a new ref
        self.use_dma = false;

        self.bcm2835_mmc_prepare_data(cmd);

        self.bcm2835_mmc_writel(cmd.arg, SDHCI_ARGUMENT, 6);

        self.bcm2835_mmc_set_transfer_mode(cmd);

        if (cmd.flags & MMC_RSP_136) != 0 && (cmd.flags & MMC_RSP_BUSY) != 0 {
            //pr_err!("{}: Unsupported response type!\n", mmc_hostname(host.mmc));
            //cmd.error = -EINVAL;
            //tasklet_schedule(&mut host.finish_tasklet);
            return Err(DevError::Io);
        }

        if (cmd.flags & MMC_RSP_PRESENT) == 0 {
            flags = SDHCI_CMD_RESP_NONE;
        } else if (cmd.flags & MMC_RSP_136) != 0 {
            flags = SDHCI_CMD_RESP_LONG;
        } else if (cmd.flags & MMC_RSP_BUSY) != 0 {
            flags = SDHCI_CMD_RESP_SHORT_BUSY;
        } else {
            flags = SDHCI_CMD_RESP_SHORT;
        }

        if (cmd.flags & MMC_RSP_CRC) != 0 {
            flags |= SDHCI_CMD_CRC;
        }
        if (cmd.flags & MMC_RSP_OPCODE) != 0 {
            flags |= SDHCI_CMD_INDEX;
        }

        if cmd.data.is_some() {
            flags |= SDHCI_CMD_DATA;
        }

        self.bcm2835_mmc_writew(SDHCI_MAKE_CMD(cmd.opcode, flags) as i32, SDHCI_COMMAND);
        Ok(())
    }
    fn bcm2835_mmc_request(&mut self, mrq: &mut MmcRequest) -> DevResult{
        //let host: &mut Bcm2835Host = mmc_priv(mmc);
        //spin_lock_irqsave(&host->lock, flags);
        //let mut spinlock = self.spinlock.lock();//todo how to save irq?
        let mut res: DevResult  = Ok(());
        {
            //WARN_ON(host.mrq.is_some());
            self.mrq = Some(SpinNoIrq::new(mrq.clone()).into());

            /*if mrq.sbc.is_some() && !(host.flags & SDHCI_AUTO_CMD23) {
                bcm2835_mmc_send_command(host, mrq.sbc.unwrap());
            } else {
                bcm2835_mmc_send_command(host, mrq.cmd);
            }*/
            if let Some(cmd) = mrq.cmd.as_mut(){
                if let  cmd_ref = cmd.lock(){
                    res = self.bcm2835_mmc_send_command(& cmd_ref);
                }
            }
        }
        /*if !(mrq.sbc.is_some() && !(host.flags & SDHCI_AUTO_CMD23)) && mrq.cmd.data.is_some() && host.use_dma {
            // DMA transfer starts now, PIO starts after interrupt
            bcm2835_mmc_transfer_dma(host);
        }*/
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
    fn mmc_request_done(&mut self, mrq: &mut MmcRequest){
        //todo
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

        /*if sdio_is_io_busy(mrq.cmd.opcode, mrq.cmd.arg) && self.card_busy {
            let mut tries = 500; // Wait approx 500ms at maximum

            while self.card_busy.unwrap()(host) && tries > 0 {
                let duration = Duration::from_micros(1000); 
                thread::sleep(duration);
                tries -= 1;
            }

            if tries == 0 {
                //mrq.cmd.error = -EBUSY;
                self.mmc_request_done(mrq);
                return Err(DevError::ResourceBusy);
            }
        }*/

        /*if mrq.cap_cmd_during_tfr {
            host.ongoing_mrq = Some(mrq);
            reinit_completion(&mrq.cmd_completion);
        }

        trace_mmc_request_start(host, mrq);

        if host.cqe_on {
            host.cqe_ops.as_ref().unwrap().cqe_off(host);
        }*/

        let res = self.bcm2835_mmc_request(mrq);
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
        let res = self.mmc_start_request(mrq);
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
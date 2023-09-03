use core::intrinsics::{volatile_load, volatile_store};

static RTC_DR: u32 = 0x000;
static RTC_MR: u32 = 0x004;
static RTC_LR: u32 = 0x008;
static RTC_CR: u32 = 0x00c;
static RTC_IMSC: u32 = 0x010;
static RTC_RIS: u32 = 0x014;
static RTC_MIS: u32 = 0x018;
static RTC_ICR: u32 = 0x01c;

pub static mut PL031_RTC: Pl031rtc = Pl031rtc {
    address: 0,
};

pub unsafe fn init() {
    PL031_RTC.init();
}

pub struct Pl031rtc {
    pub address: usize,
}

pub const PHYS_OFFSET: usize = 0xffff_0000_0000_0000;

impl Pl031rtc {
    unsafe fn init(&mut self) {
        self.address = PHYS_OFFSET + 0x09010000;
    }

    unsafe fn read(&self, reg: u32) -> u32 {
        let val = volatile_load((self.address + reg as usize) as *const u32);
        val
    }

    unsafe fn write(&mut self, reg: u32, value: u32) {
        volatile_store((self.address + reg as usize) as *mut u32, value);
    }

    pub fn time(&mut self) -> u64 {
        let seconds = unsafe { self.read(RTC_DR) } as u64;
        seconds
    }
}
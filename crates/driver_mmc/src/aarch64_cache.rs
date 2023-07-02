use core::marker::PhantomData;
use aarch64_cpu::{asm::barrier::*, registers::*};
use core::arch::asm;

pub trait CoherencyPoint {}

pub struct PoC;

pub struct PoU;

impl CoherencyPoint for PoU {}
impl CoherencyPoint for PoC {}

pub trait Flush {}

pub struct Clean;

/// Invalidate old data in the cache
pub struct Invalidate;

/// A clean instruction followed by a invalidate instruction
pub struct CleanAndInvalidate;

impl Flush for Clean {}
impl Flush for Invalidate {}
impl Flush for CleanAndInvalidate {}


pub trait Cache {
    /// Flush a cache line by the virtual address.
    fn flush_line_op(vaddr: usize);
    /// Cache line size in bytes
    fn cache_line_size() -> u64;

    /// Flush cache for the VA interval [start, end) in the shareability domain.
    fn flush_range<A: sealed::Dsb>(start: usize, end: usize, domain: A) {
        let line_size = 4 << Self::cache_line_size();
        let mut addr = start & !(line_size - 1);
        while addr < end {
            Self::flush_line_op(addr);
            addr += line_size;
        }
        unsafe { dsb(domain) };
        unsafe { isb(SY) };
    }

    /// Flush cache for the VA interval [start, start + sizeend) in the
    /// shareability domain.
    fn flush_area<A: sealed::Dsb>(start: usize, size: usize, domain: A) {
        Self::flush_range(start, start + size, domain);
    }
}

pub struct ICache<F: Flush = Invalidate, P: CoherencyPoint = PoU> {
    _f: PhantomData<F>,
    _p: PhantomData<P>,
}

pub struct DCache<F: Flush, P: CoherencyPoint> {
    _f: PhantomData<F>,
    _p: PhantomData<P>,
}

impl ICache {
    /// Invalidate all I-Cache to the Point of Unification in all PEs.
    #[inline]
    pub fn flush_all() {
        //unsafe { llvm_asm!("ic ialluis; dsb ish; isb":::: "volatile") };
        unsafe {
            asm!(
                "ic ialluis; dsb ish; isb",
                options(nostack, nomem, preserves_flags)
            );
        }
    }
    /// Invalidate all I-Cache to the Point of Unification in the current PE.
    #[inline]
    pub fn local_flush_all() {
        //unsafe { llvm_asm!("ic iallu; dsb nsh; isb":::: "volatile") };
        unsafe {
            asm!(
                "ic iallu; dsb nsh; isb",
                options(nostack, nomem, preserves_flags)
            );
        }
    }
}

macro_rules! cache_ins {
    (ICache) => {
        "ic"
    };
    (DCache) => {
        "dc"
    };
}

macro_rules! cache_op {
    (Clean) => {
        "c"
    };
    (Invalidate) => {
        "i"
    };
    (CleanAndInvalidate) => {
        "ci"
    };
}

macro_rules! cache_point {
    (PoC) => {
        "c"
    };
    (PoU) => {
        "u"
    };
}

#[inline(always)]
unsafe fn read_ctr_el0() -> u64 {
    let val: u64;
    asm!("mrs {}, ctr_el0", out(reg) val, options(nostack, preserves_flags));
    val
}


macro_rules! cache_line_size {
    (ICache) => {
        //CTR_EL0::IminLine
        unsafe{((read_ctr_el0() >> 0) & 0xF)}//3_0
    };
    (DCache) => {
        //CTR_EL0::DminLine
        unsafe{((read_ctr_el0() >> 16) & 0xF)}//19-16
    };
}

impl Cache for DCache<Clean, PoC> {
    #[inline]
    fn flush_line_op(vaddr: usize) {
        unsafe {
            asm!(
                concat!(
                    cache_ins!(DCache),
                    " ",
                    cache_op!(Clean),
                    "va",
                    cache_point!(PoC),
                    ", {0}"
                ),
                in(reg) vaddr,
                options(nomem, nostack)
            );
        }
    }
    #[inline]
    fn cache_line_size() -> u64 {
        cache_line_size!(DCache)
    }
}

impl Cache for DCache<Invalidate, PoC> {
    #[inline]
    fn flush_line_op(vaddr: usize) {
        unsafe {
            asm!(
                concat!(
                    cache_ins!(DCache),
                    " ",
                    cache_op!(Invalidate),
                    "va",
                    cache_point!(PoC),
                    ", {0}"
                ),
                in(reg) vaddr,
                options(nomem, nostack)
            );
        }
    }
    #[inline]
    fn cache_line_size() -> u64 {
        cache_line_size!(DCache)
    }
}



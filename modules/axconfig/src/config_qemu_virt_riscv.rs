//! Platform constants and parameters for qemu-virt-riscv.
//! Generated by build.rs, DO NOT edit!

/// Stack size of each task.
pub const TASK_STACK_SIZE: usize = 0x40000;
/// Number of timer ticks per second (Hz). A timer tick may contain several timer
/// interrupts.
pub const TICKS_PER_SEC: usize = 100;
/// Architecture identifier.
pub const ARCH: &str = "riscv64";
/// Platform identifier.
pub const PLATFORM: &str = "qemu-virt-riscv";
/// Base address of the whole physical memory.
pub const PHYS_MEMORY_BASE: usize = 0x8000_0000;
/// Size of the whole physical memory.
pub const PHYS_MEMORY_SIZE: usize = 0x800_0000;
/// Base physical address of the kernel image.
pub const KERNEL_BASE_PADDR: usize = 0x8020_0000;
/// Base virtual address of the kernel image.
pub const KERNEL_BASE_VADDR: usize = 0xffff_ffc0_8020_0000;
/// Linear mapping offset, for quick conversions between physical and virtual
/// addresses.
pub const PHYS_VIRT_OFFSET: usize = 0xffff_ffc0_0000_0000;
/// MMIO regions with format (`base_paddr`, `size`).
pub const MMIO_REGIONS: &[(usize, usize)] = &[
    (0x0c00_0000, 0x21_0000),
    (0x1000_0000, 0x1000),
    (0x1000_1000, 0x8000),
    (0x3000_0000, 0x1000_0000),
    (0x4000_0000, 0x4000_0000),
];
/// VirtIO MMIO regions with format (`base_paddr`, `size`).
pub const VIRTIO_MMIO_REGIONS: &[(usize, usize)] = &[
    (0x1000_1000, 0x1000),
    (0x1000_2000, 0x1000),
    (0x1000_3000, 0x1000),
    (0x1000_4000, 0x1000),
    (0x1000_5000, 0x1000),
    (0x1000_6000, 0x1000),
    (0x1000_7000, 0x1000),
    (0x1000_8000, 0x1000),
];
/// Base physical address of the PCIe ECAM space.
pub const PCI_ECAM_BASE: usize = 0x3000_0000;
/// End PCI bus number (`bus-range` property in device tree).
pub const PCI_BUS_END: usize = 0xff;
/// PCI device memory ranges (`ranges` property in device tree).
pub const PCI_RANGES: &[(usize, usize)] = &[
    (0x0300_0000, 0x1_0000),
    (0x4000_0000, 0x4000_0000),
    (0x4_0000_0000, 0x4_0000_0000),
];
/// Timer interrupt frequency in Hz.
pub const TIMER_FREQUENCY: usize = 10_000_000;
/// Number of CPUs
pub const SMP: usize = 1;

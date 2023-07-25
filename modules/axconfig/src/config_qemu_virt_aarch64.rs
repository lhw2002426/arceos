//! Platform constants and parameters for qemu-virt-aarch64.
//! Generated by build.rs, DO NOT edit!

/// Stack size of each task.
pub const TASK_STACK_SIZE: usize = 0x40000;
/// Number of timer ticks per second (Hz). A timer tick may contain several timer
/// interrupts.
pub const TICKS_PER_SEC: usize = 100;
/// Architecture identifier.
pub const ARCH: &str = "aarch64";
/// Platform identifier.
pub const PLATFORM: &str = "qemu-virt-aarch64";
/// Base address of the whole physical memory.
pub const PHYS_MEMORY_BASE: usize = 0x4000_0000;
/// Size of the whole physical memory.
pub const PHYS_MEMORY_SIZE: usize = 0x800_0000;
/// Base physical address of the kernel image.
///kernel-base-paddr = "0x4008_0000"
pub const KERNEL_BASE_PADDR: usize = 0x0008_0000;
/// Base virtual address of the kernel image.
///kernel-base-vaddr = "0xffff_0000_4008_0000"
pub const KERNEL_BASE_VADDR: usize = 0x0000_0000_0008_0000;
/// Linear mapping offset, for quick conversions between physical and virtual
/// addresses.
///phys-virt-offset = "0xffff_0000_0000_0000"
pub const PHYS_VIRT_OFFSET: usize = 0x0000_0000_0000_0000;
/// MMIO regions with format (`base_paddr`, `size`).
pub const MMIO_REGIONS: &[(usize, usize)] = &[
    (0x0900_0000, 0x1000),
    (0x0800_0000, 0x2_0000),
    (0x0a00_0000, 0x4000),
    (0x1000_0000, 0x2eff_0000),
    (0x40_1000_0000, 0x1000_0000),
];
/// VirtIO MMIO regions with format (`base_paddr`, `size`).
pub const VIRTIO_MMIO_REGIONS: &[(usize, usize)] = &[
    (0x0a00_0000, 0x200),
    (0x0a00_0200, 0x200),
    (0x0a00_0400, 0x200),
    (0x0a00_0600, 0x200),
    (0x0a00_0800, 0x200),
    (0x0a00_0a00, 0x200),
    (0x0a00_0c00, 0x200),
    (0x0a00_0e00, 0x200),
    (0x0a00_1000, 0x200),
    (0x0a00_1200, 0x200),
    (0x0a00_1400, 0x200),
    (0x0a00_1600, 0x200),
    (0x0a00_1800, 0x200),
    (0x0a00_1a00, 0x200),
    (0x0a00_1c00, 0x200),
    (0x0a00_1e00, 0x200),
    (0x0a00_3000, 0x200),
    (0x0a00_2200, 0x200),
    (0x0a00_2400, 0x200),
    (0x0a00_2600, 0x200),
    (0x0a00_2800, 0x200),
    (0x0a00_2a00, 0x200),
    (0x0a00_2c00, 0x200),
    (0x0a00_2e00, 0x200),
    (0x0a00_3000, 0x200),
    (0x0a00_3200, 0x200),
    (0x0a00_3400, 0x200),
    (0x0a00_3600, 0x200),
    (0x0a00_3800, 0x200),
    (0x0a00_3a00, 0x200),
    (0x0a00_3c00, 0x200),
    (0x0a00_3e00, 0x200),
];
/// Base physical address of the PCIe ECAM space.
pub const PCI_ECAM_BASE: usize = 0x40_1000_0000;
/// End PCI bus number (`bus-range` property in device tree).
pub const PCI_BUS_END: usize = 0xff;
/// PCI device memory ranges (`ranges` property in device tree).
pub const PCI_RANGES: &[(usize, usize)] = &[
    (0x3ef_f0000, 0x1_0000),
    (0x1000_0000, 0x2eff_0000),
    (0x80_0000_0000, 0x80_0000_0000),
];
/// UART Address
pub const UART_PADDR: usize = 0x0900_0000;

pub const UART_IRQ_NUM: usize = 33;
/// GICC Address
pub const GICC_PADDR: usize = 0x0801_0000;

pub const GICD_PADDR: usize = 0x0800_0000;
/// Number of CPUs
pub const SMP: usize = 1;

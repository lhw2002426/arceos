//test.rs
use std::sync::Arc;


use crate::bcm2835::SDHCIdevice;
use driver_common::{BaseDriverOps, DevError, DevResult, DeviceType};
use driver_block::BlockDriverOps;

#[test]
fn test_mmc() {
    print!("hello mmc!");
    let mut sdhci_mmc = SDHCIdevice::new();
    let a = sdhci_mmc.num_blocks();
    let empty_slice: &mut [u8] = &mut [];
    sdhci_mmc.read_block(0, empty_slice);
    assert_eq!(a,1);
}
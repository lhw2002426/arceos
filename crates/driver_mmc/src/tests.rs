//test.rs
use std::sync::Arc;


use crate::bcm2835::SDHCIdevice;
use driver_common::{BaseDriverOps, DevError, DevResult, DeviceType};
use driver_block::BlockDriverOps;

#[test]
fn test_mmc() {
    print!("hello mmc!");
    let sdhci_mmc = SDHCIdevice::new();
    let a = sdhci_mmc.num_blocks();
    assert_eq!(a,0);
}
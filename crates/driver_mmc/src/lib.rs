//! sd card driver for raspi4, 
//! if you want to use it, change [dev-dependencies] in modules/axfs/Cargo.toml 
//! from axdriver = { path = "../axdriver", features = ["block", "ramdisk"] } to axdriver = { path = "../axdriver", features = ["block", "mmc"] }
//! ans change [features] in  modules/axruntime/Cargo.toml
//! from "fs = ["alloc", "paging", "axdriver/virtio-blk", "dep:axfs"]" to "fs = ["alloc", "paging", "axdriver/mmc", "dep:axfs"]

#![no_std]
#![feature(doc_auto_cfg)]
#![feature(const_trait_impl)]

#[macro_use]
extern crate log;
pub mod bcm2835_sdhci;
//pub mod mailbox;
//pub mod aarch64_cache;

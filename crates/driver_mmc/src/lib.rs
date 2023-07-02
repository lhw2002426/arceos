//! Common traits and types for block storage device drivers (i.e. disk).

#![no_std]
#![feature(doc_auto_cfg)]
#![feature(const_trait_impl)]

#[macro_use]
extern crate log;
pub mod bcm2835_sdhci;
pub mod mailbox;
pub mod aarch64_cache;

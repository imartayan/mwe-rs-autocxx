#![allow(dead_code)]

// pub use cxx::UniquePtr;
// use std::fmt::Debug;

// #[cxx::bridge]
// mod tv28 {
//     unsafe extern "C++" {
//         include!("mwe/cxx/tiered_vec.h");

//         type TieredVec28;
//         fn new_tiered_vec_28() -> UniquePtr<TieredVec28>;
//         fn len(&self) -> usize;
//         fn is_empty(&self) -> bool;
//         fn capacity(&self) -> usize;
//         fn get(&self, idx: usize) -> u32;
//         fn update(&self, idx: usize, elem: u32) -> u32;
//         fn insert(&self, idx: usize, elem: u32);
//         fn remove(&self, idx: usize);
//         fn insert_sorted(&self, elem: u32);
//         fn contains_sorted(&self, elem: u32) -> bool;
//         fn index_sorted(&self, elem: u32) -> usize;
//     }
// }
// pub use tv28::*;

// impl Debug for TieredVec28 {
//     fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
//         Ok(())
//     }
// }

use autocxx::prelude::*; // use all the main autocxx functions

include_cpp! {
    // #include "tiered_vec.h"
    #include "tiered_vec.cc"
    safety!(unsafe)
    generate!("TieredVec28") // add this line for each function or type you wish to generate
    // generate!("new_tiered_vec_28") // add this line for each function or type you wish to generate
}

pub use ffi::*;

#![allow(dead_code)]
#![allow(unused_imports)]

// pub use cxx::UniquePtr;
// pub use rbv::*;

// #[cxx::bridge]
// mod rbv {
//     unsafe extern "C++" {
//         include!("mwe/cxx/rank_bv.h");

//         type RankBV;
//         fn new_rank_bv(size: usize) -> UniquePtr<RankBV>;
//         fn size(&self) -> usize;
//         fn get(&self, index: usize) -> bool;
//         fn set(&self, index: usize) -> bool;
//         fn clear(&self, index: usize) -> bool;
//         fn toggle(&self, index: usize) -> bool;
//         fn rank(&self, index: usize) -> u64;
//         fn count_ones(&self) -> usize;
//     }
// }

use autocxx::prelude::*; // use all the main autocxx functions

include_cpp! {
    // #include "rank_bv.h"
    #include "rank_bv.cc"
    generate!("RankBV") // add this line for each function or type you wish to generate
    // generate!("new_rank_bv") // add this line for each function or type you wish to generate
    safety!(unsafe)
}

pub use ffi::*;

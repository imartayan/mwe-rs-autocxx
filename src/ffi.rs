#![allow(dead_code)]
#![allow(unused_imports)]

use autocxx::prelude::*;

include_cpp! {
    #include "rank_bv.cc"
    #include "tiered_vec.cc"
    generate!("RankBV")
    generate!("TieredVec16")
    generate!("TieredVec20")
    generate!("TieredVec24")
    generate!("TieredVec28")
    generate!("TieredVec32")

    safety!(unsafe)
}

pub use ffi::*;

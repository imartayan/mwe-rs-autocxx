#![allow(unused_imports)]
// #![allow(incomplete_features)]
// #![feature(slice_group_by)]
// #![feature(generic_const_exprs)]

mod rank_bv;
// mod tiered_vec;

// use autocxx::prelude::*;
use autocxx::WithinUniquePtr;

fn main() {
    println!("Hi ffi!");

    // let rbv = rank_bv::new_rank_bv(20);
    let rbv = rank_bv::RankBV::new(20).within_unique_ptr();
    rbv.set(12);
    assert!(rbv.get(12));

    // // let tv = tiered_vec::new_tiered_vec_28();
    // // let tv = tiered_vec::TieredVec28::new();
    // let tv = tiered_vec::TieredVec28::new().within_unique_ptr();
    // tv.insert(0, 42);
    // assert_eq!(tv.get(0), 42);

    println!("All set!");
}

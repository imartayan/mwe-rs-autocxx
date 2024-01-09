#![allow(unused_imports)]

mod ffi;

use autocxx::WithinUniquePtr;
use ffi::*;

fn main() {
    println!("Hi ffi!");

    let rbv = RankBV::new(20).within_unique_ptr();
    rbv.set(12);
    assert!(rbv.get(12));

    let tv = TieredVec28::new().within_unique_ptr();
    tv.insert(0, 42);
    assert_eq!(tv.get(0), 42);

    println!("All set!");
}

#![feature(stdarch_x86_avx512)]
#![feature(avx512_target_feature)]

pub mod days;
pub use days::*;

type AdventHashMap<K, V> = ahash::AHashMap<K, V>;

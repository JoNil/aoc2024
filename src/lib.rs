#![feature(stdarch_x86_avx512)]
#![feature(avx512_target_feature)]
#![feature(map_many_mut)]

pub mod days;
pub use days::*;

pub type AdventHashMap<K, V> = rustc_hash::FxHashMap<K, V>;
pub type AdventHashSet<K> = rustc_hash::FxHashSet<K>;

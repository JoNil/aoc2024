#![feature(stdarch_x86_avx512)]
#![feature(avx512_target_feature)]

pub mod days;
pub use days::*;

type AdventHashMap<K, V> = rustc_hash::FxHashMap<K, V>;
type AdventHashSet<K> = rustc_hash::FxHashSet<K>;

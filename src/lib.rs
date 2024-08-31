#![no_std]
pub use apihasher_procs;

#[macro_export]
macro_rules! api_hash_consts {
    ($($fnname:literal),+) => {
        $crate::apihasher_procs::gen_key!();
        $(
            $crate::apihasher_procs::gen_const!($fnname);
         )+
    };
}

#[macro_export]
macro_rules! api_hash_structs {
    ($($fnname:literal),+) => {
        $(
            $crate::apihasher_procs::gen_struct!($fnname);
         )+
    };
}

pub struct ApiHasher {
    pub key: u64,
    pub hash: u64,
}


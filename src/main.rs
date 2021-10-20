use mc_oblivious_ram::{PathORAM4096Z4Creator};
use mc_oblivious_traits::ORAMCreator;
use aligned_cmov::{A64Bytes, A8Bytes, ArrayLength};
use mc_oblivious_traits::{rng_maker, testing, HeapORAMStorageCreator, ORAM};
use mc_fog_ocall_oram_storage_trusted::OcallORAMStorageCreator;
pub use mc_fog_ocall_oram_storage_untrusted::{
        allocate_oram_storage, checkin_oram_storage, checkout_oram_storage, release_oram_storage,
};
use rand_hc::Hc128Rng;
use rand_core::SeedableRng;

pub type RngType = Hc128Rng;
pub fn get_seeded_rng() -> RngType {
            RngType::from_seed([7u8; 32])
}
fn a64_bytes<N: ArrayLength<u8>>(src: u8) -> A64Bytes<N> {
    let mut result = A64Bytes::<N>::default();
    for byte in result.iter_mut() {
        *byte = src;
    }
    result
}
fn main() {
    let mut maker = rng_maker(get_seeded_rng());
    const STASH_SIZE: usize = 16;
    println!("Hello Worldo!");
    //let mut oram = PathORAM4096Z4Creator::<RngType, HeapORAMStorageCreator>::create(
    let mut oram = PathORAM4096Z4Creator::<RngType, OcallORAMStorageCreator>::create(
        8192,
        STASH_SIZE,
        &mut maker,
    );
    assert_eq!(a64_bytes(0), oram.write(0, &a64_bytes(2)));
    assert_eq!(a64_bytes(2), oram.write(0, &a64_bytes(0)));
    println!("okbye");

}
use mc_oblivious_ram::{PathORAM4096Z4Creator};
use mc_oblivious_traits::ORAMCreator;
use aligned_cmov::{A64Bytes, ArrayLength};
//use mc_oblivious_traits::{rng_maker, testing, HeapORAMStorageCreator, ORAM};
use mc_oblivious_traits::{rng_maker, ORAM};
use mc_fog_ocall_oram_storage_trusted::OcallORAMStorageCreator;
pub use mc_fog_ocall_oram_storage_untrusted::{
        allocate_oram_storage, checkin_oram_storage, checkout_oram_storage, release_oram_storage,
};
//use mc_fog_ledger_enclave_api::KeyImageData;
use mc_transaction_core::ring_signature::KeyImage;
use rand_hc::Hc128Rng;
use rand_core::SeedableRng;

use mc_fog_ledger_enclave::{LedgerSgxEnclave, ENCLAVE_FILE, LedgerEnclave};
use mc_common::{
    logger::{create_app_logger, log, o},
    //time::SystemTimeProvider,
    ResponderId,
};
use std::{env};
use std::vec::Vec;


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
    //Non-sgx test
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
    
    //sgx
    mc_common::setup_panic_handler();
    let (logger, _global_logger_guard) = create_app_logger(o!());
    let enclave_path = env::current_exe()
        .expect("Could not get the path of our executable")
        .with_file_name(ENCLAVE_FILE);
    //let responder_id = ResponderId("123.123.123.0:9004".to_string());
    let enclave = LedgerSgxEnclave::new(
        enclave_path,
        //&responder_id,
        65536,
        logger.clone(),
    );
    /*let rec = KeyImageData {
        key_image: KeyImage::from(2),
        block_index: 15968249514437158236,
        timestamp: 14715610560481527175,
    };*/
    //must be 32
    let key: Vec<u8> = Vec::from([2,3,5,2,3,5,2,3,5,2,3,5,2,3,5,2,3,5,2,3,5,2,3,5,2,3,5,2,3,5,2,3]);
    //must be 16
    let value: Vec<u8> = Vec::from([4,6,121,42,4,6,121,42,4,6,121,42,4,6,121,42]);
    let s1 = String::from_utf8(value.to_vec()).unwrap();
    enclave.add_omap_item(key.to_vec(), value.to_vec()).unwrap();
    let retrieved_value = enclave.get_omap_item(key.to_vec()).unwrap();
    assert_eq!(value.to_vec(), retrieved_value.to_vec());
    let s = String::from_utf8(retrieved_value).unwrap();

    
    enclave.add_oram_item(0, Vec::from([2; 1024])).unwrap();
    assert_eq!(Vec::from([2; 1024]), enclave.get_oram_item(0).unwrap());
    println!("okbye");

}

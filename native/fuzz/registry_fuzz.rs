#![no_main]
use libfuzzer_sys::fuzz_target;
use zap_native::parse_index_bytes;

fuzz_target!(|data: &[u8]| {
    let _ = parse_index_bytes(data);
});
#![no_main]
use libfuzzer_sys::fuzz_target;
use zap_native::parse_program;

fuzz_target!(|data: &[u8]| {
    if let Ok(source) = std::str::from_utf8(data) {
        let _ = parse_program(source);
    }
});
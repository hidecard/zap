#![no_main]
use libfuzzer_sys::fuzz_target;
use zap_native::parse_resolved_lockfile;

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = std::str::from_utf8(data) {
        let _ = parse_resolved_lockfile(text);
    }
});
#![no_main]
use libfuzzer_sys::fuzz_target;
use zap_native::json_to_value;
use serde_json;

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = std::str::from_utf8(data) {
        // Test both the Zap JSON parser and serde_json directly
        let _ = serde_json::from_str::<serde_json::Value>(text);
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(text) {
            let _ = json_to_value(parsed);
        }
    }
});
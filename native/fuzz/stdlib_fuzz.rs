#![no_main]
use libfuzzer_sys::fuzz_target;
use zap_native::{parse_program, execute_ast_program_with_context, ExecutionContext};

fuzz_target!(|data: &[u8]| {
    if let Ok(source) = std::str::from_utf8(data) {
        if let Ok(program) = parse_program(source) {
            let mut context = ExecutionContext::new();
            let _ = execute_ast_program_with_context(&program, &mut context);
        }
    }
});
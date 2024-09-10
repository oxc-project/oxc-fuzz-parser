#![no_main]

use oxc::allocator::Allocator;
use oxc_regular_expression::{ParserOptions, PatternParser};

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        if s.chars().all(|s| !s.is_control()) {
            let allocator = Allocator::default();
            let _ = PatternParser::new(
                &allocator,
                &s,
                ParserOptions {
                    span_offset: 0,
                    unicode_mode: true,
                    unicode_sets_mode: true,
                }
            ).parse();
        }
    }
});

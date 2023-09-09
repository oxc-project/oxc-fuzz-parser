use std::{env, path::Path, process};

use oxc::{allocator::Allocator, parser::Parser, span::SourceType};

fn main() {
    let name = env::args().nth(1).unwrap();
    let path = Path::new(&name);
    let source_text = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("{name} not found"));
    let allocator = Allocator::default();
    let source_type = SourceType::from_path(path).unwrap();
    let ret = Parser::new(&allocator, &source_text, source_type).parse();

    for error in ret.errors {
        let error = error.with_source_code(source_text.clone());
        eprintln!("{error:?}");
        process::exit(1);
    }
}

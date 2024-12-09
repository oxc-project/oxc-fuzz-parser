use std::{
    env, mem,
    path::{Path, PathBuf},
};

use oxc::{
    codegen::{CodegenOptions, CodegenReturn},
    diagnostics::{Error, OxcDiagnostic},
    minifier::CompressOptions,
    parser::ParseOptions,
    span::SourceType,
    transformer::TransformOptions,
    CompilerInterface,
};

fn main() -> Result<(), String> {
    let name = env::args().nth(1).unwrap();
    let path = Path::new(&name);

    let source_text = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("{name} not found"));
    let source_type = SourceType::from_path(path).unwrap();

    let s1 = Driver::default()
        .run(path, &source_text, source_type)
        // .inspect_err(|err| panic!("{err:?}"))
        .unwrap_or_default();

    Driver::default()
        .run(path, &s1, source_type)
        .map(|_| ())
        .map_err(|err| panic!("{err:?}"))
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Default)]
pub struct Driver {
    // options
    pub transform: Option<TransformOptions>,
    pub compress: Option<CompressOptions>,
    pub mangle: bool,
    pub remove_whitespace: bool,
    // states
    pub printed: String,
    pub path: PathBuf,
    pub errors: Vec<OxcDiagnostic>,
}

impl CompilerInterface for Driver {
    fn handle_errors(&mut self, errors: Vec<OxcDiagnostic>) {
        self.errors.extend(errors);
    }

    fn after_codegen(&mut self, ret: CodegenReturn) {
        self.printed = ret.code;
    }

    fn parse_options(&self) -> ParseOptions {
        ParseOptions {
            parse_regular_expression: true,
            allow_return_outside_function: true,
            ..ParseOptions::default()
        }
    }

    fn transform_options(&self) -> Option<&TransformOptions> {
        None
    }

    fn compress_options(&self) -> Option<CompressOptions> {
        None
    }

    fn codegen_options(&self) -> Option<CodegenOptions> {
        Some(CodegenOptions {
            ..CodegenOptions::default()
        })
    }
}

impl Driver {
    pub fn run(
        &mut self,
        source_path: &Path,
        source_text: &str,
        source_type: SourceType,
    ) -> Result<String, Vec<Error>> {
        self.path = source_path.to_path_buf();
        self.compile(source_text, source_type, source_path);
        if self.errors.is_empty() {
            Ok(mem::take(&mut self.printed))
        } else {
            let errors = mem::take(&mut self.errors)
                .into_iter()
                .map(|error| error.with_source_code(source_text.to_string()))
                .collect();
            Err(errors)
        }
    }
}

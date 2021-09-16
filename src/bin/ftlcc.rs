use std::{fs, env};
use std::io::{stdin, Read, stdout, BufWriter, Write};
use fortytwo_lang::parser::sourcecode_to_parser;
use fortytwo_lang::emitter_c::EmitterC;
use std::path::Path;
use std::fs::File;

fn main() {
    let args: Vec<_> = env::args().collect();
    match args.get(1) {
        Some(arg) if arg == "--help" => show_help(),
        Some(filepath) => compile_from_filepath(Path::new(filepath)),
        None => compile_from_stdin(),
    };
}

/// Reads sourcecode from a file and compiles it.
fn compile_from_filepath(filepath: &Path) {
    let sourcecode = fs::read_to_string(filepath)
        .expect("Could not read sourcecode from specified file");
    let target_path = filepath.with_extension("c");
    let target_file = File::create(target_path).expect("Could not create target file");
    compile(sourcecode, target_file);
}

/// Reads sourcecode from stdin until an EOF is received and compiles it.
fn compile_from_stdin() {
    let mut sourcecode = String::new();
    stdin().read_to_string(&mut sourcecode)
        .expect("Could not read sourcecode from stdin");
    let target = stdout();
    compile(sourcecode, target);
}

/// Compiles `sourcecode` and writes the emitted c sourcecode into `emit_target`.
fn compile(sourcecode: String, emit_target: impl Write) {
    let parser = sourcecode_to_parser(sourcecode.chars());
    let ast_nodes = parser.map(|ast_node| ast_node.unwrap());
    EmitterC::codegen(ast_nodes, BufWriter::new(emit_target)).unwrap();
}

fn show_help() {
    println!(
        r#"FORTYTWO-LANG C COMPILER
Compiles ftl sourcecode to c sourcecode.

USAGE:
    ftlcc [FILE]

ARGUMENTS:
    - FILE: The path to the file you want to compile. ftlcc generates
    a file with the same name, but the extension `.c`.
    If FILE is not specified, the sourcecode is read from stdin until
    an EOF is read. The compiled c code is then printed to stdout.
"#
    )
}
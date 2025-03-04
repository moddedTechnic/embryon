use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    input: PathBuf,
}

fn main() {
    let args = Args::parse();
    let source = std::fs::read_to_string(&args.input).unwrap();
    let tokens = embryon_lang::lex(&source);
    let program = embryon_lang::parse(tokens).unwrap();
    embryon_lang::compile(program, &args.input);

    let bc = args.input.with_extension("bc");
    let asm = args.input.with_extension("s");

    // run `llc` to generate assembly
    let _ = std::process::Command::new("llc")
        .arg(bc)
        .arg("-o")
        .arg(asm)
        .output()
        .unwrap();

    // Compile assembly
    // arm-none-eabi-gcc -o examples/simple_fn.elf examples/simple_fn.s startup.s -nostartfiles -nodefaultlibs -T./microbit.ld
}

use std::os::unix::prelude::CommandExt;
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
    let program = embryon_lang::parse(tokens).expect("failed to compile program");
    embryon_lang::compile(program, &args.input);

    let asm = args.input.with_extension("s");

    // Compile assembly
    // TODO: should probably use some form of integrated gcc
    let elf = args.input.with_extension("elf");
    let _ = std::process::Command::new("arm-none-eabi-gcc")
        .arg("-o")
        .arg(&elf)
        .arg(&asm)
        .arg("startup.s")
        .arg("-nostartfiles")
        .arg("-nodefaultlibs")
        .arg("-T./microbit.ld")
        .exec();
}

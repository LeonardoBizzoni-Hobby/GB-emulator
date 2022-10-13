use std::{process, env};

mod emulator;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if let Err(msg) = emulator::emu_run(&args) {
	eprintln!("{}", msg);
	process::exit(1);
    }
}

use std::{process, env};

mod cpu;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if let Err(msg) = cpu::emulator::emu_run(&args) {
	eprintln!("{}", msg);
	process::exit(1);
    }
}

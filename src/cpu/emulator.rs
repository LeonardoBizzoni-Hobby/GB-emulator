use colored::Colorize;
use sdl2::pixels::Color;

pub fn emu_run(args: &Vec<String>) -> Result<(), &'static str>{
    if args.len() < 2 {
	return Err("Usage: lb-emu <rom_file>");
    }

    // Future use
    let _rom = match cartrige_loaded(&args[1]) {
	Ok(rom) => rom,
	Err(msg) => return Err(msg)
    };

    println!("{}", format!("Cartrige loaded successfully!").green().bold());

    // Graphics Library: https://docs.rs/sdl2/0.35.2/sdl2/index.html#getting-started
    // Replace unwrap with match statements
    let sdl_context = sdl2::init().unwrap();
    let _video_subsystem = sdl_context.video().unwrap();

    // println!("{}", format!("SDL initialized successfully!").green().bold());

    Ok(())
}

fn cartrige_loaded(_rom: &str) -> Result<(), &'static str>{
    // TODO: check if rom file exists and return it.
    Ok(())
}

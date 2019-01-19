use std::env;
use std::process;

mod chip8;
use self::chip8::Chip8;

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("A file has to be given as argument.");
        process::exit(0x0100);
    } else {
        let mut chip = Chip8::new();
        let filename = &args[1];
        match chip.load_file(filename) {
            Ok(_) => chip.play(),
            Err(err) => println!("{}", err),
        }
    }
}

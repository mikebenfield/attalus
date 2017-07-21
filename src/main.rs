#![recursion_limit = "1024"]

extern crate attalus;

use attalus::emulation_manager::*;
use attalus::log::*;
use attalus::hardware::memory_mapper::implementation::*;

fn main() {
    let log = LogEverything::new(std::io::stdout());
    let mut args: Vec<String> = Vec::new();
    args.extend(std::env::args());
    if args.len() < 3 {
        eprintln!("Usage: exec filename n");
        return;
    }
    let filename = &args[1];
    let smmh =
        <SegaMemoryMapperHardware as MemoryMapperHardware>::
            new_from_file(filename.clone(), 0x2000).unwrap();

    let mut em = EmulationManager::new(log, smmh);

    let n: u32 = args[2].parse().expect("Usage: exec filename n");

    main_loop(&mut em, n);
}
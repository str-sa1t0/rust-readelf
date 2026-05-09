mod cli;
mod printers;
mod types;

use clap::Parser;
use cli::Args;
use elf::endian::AnyEndian;
use elf::ElfStream;
use std::path::PathBuf;

fn main() {
    let args = Args::parse();

    let path: PathBuf = args.file_name.into();

    let io = std::fs::File::open(path).expect("Could not open file");

    let mut elf_file =
        ElfStream::<AnyEndian, _>::open_stream(io).expect("Failed to open ELF stream");

    if args.file_header {
        printers::file_header::print(&elf_file.ehdr);
    }

    if args.program_headers {
        printers::program_headers::print(&mut elf_file);
    }

    if args.section_headers {
        printers::section_headers::print(&mut elf_file);
    }

    if args.symbols {
        printers::symbols::print_symbol_table(&mut elf_file);
    }

    if args.dynamic_symbols {
        printers::symbols::print_dynamic_symbol_table(&mut elf_file);
    }

    if args.dynamic {
        printers::dynamic::print(&mut elf_file);
    }

    if args.notes {
        printers::notes::print(&mut elf_file);
    }

    if args.relocations {
        printers::relocations::print(&mut elf_file);
    }
}
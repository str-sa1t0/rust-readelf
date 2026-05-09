use comfy_table::{Cell, Table};
use elf::endian::AnyEndian;
use elf::relocation::{RelIterator, RelaIterator};

use crate::types::ElfFile;

pub fn print(elf_file: &mut ElfFile) {
    let shdrs: Vec<elf::section::SectionHeader> = elf_file
        .section_headers()
        .iter()
        .filter(|shdr| {
            shdr.sh_type == elf::abi::SHT_REL || shdr.sh_type == elf::abi::SHT_RELA
        })
        .copied()
        .collect();

    for shdr in &shdrs {
        if shdr.sh_type == elf::abi::SHT_REL {
            let rels = elf_file
                .section_data_as_rels(shdr)
                .expect("Failed to read relocation section");

            print_rels(rels);
        } else if shdr.sh_type == elf::abi::SHT_RELA {
            let relas = elf_file
                .section_data_as_relas(shdr)
                .expect("Failed to read relocation section");

            print_relas(relas);
        }
    }
}

fn print_rels(rels: RelIterator<AnyEndian>) {
    let mut table = Table::new();

    table.set_header(["r_type", "r_sym", "r_offset"]);

    for r in rels {
        let cells: Vec<Cell> = vec![
            format!("{:#X?}", r.r_type).into(),
            format!("{:#X?}", r.r_sym).into(),
            format!("{:#X?}", r.r_offset).into(),
        ];

        table.add_row(cells);
    }

    println!("{table}");
}

fn print_relas(relas: RelaIterator<AnyEndian>) {
    let mut table = Table::new();

    table.set_header(["r_type", "r_sym", "r_offset", "r_addend"]);

    for r in relas {
        let cells: Vec<Cell> = vec![
            format!("{:#X?}", r.r_type).into(),
            format!("{:#X?}", r.r_sym).into(),
            format!("{:#X?}", r.r_offset).into(),
            format!("{:#X?}", r.r_addend).into(),
        ];

        table.add_row(cells);
    }

    println!("{table}");
}
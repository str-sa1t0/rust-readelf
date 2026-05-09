use comfy_table::{Cell, Table};

use crate::types::ElfFile;

pub fn print(elf_file: &mut ElfFile) {
    let mut table = Table::new();

    table.set_header([
        "p_type",
        "p_offset",
        "p_vaddr",
        "p_paddr",
        "p_filesz",
        "p_memsz",
        "p_flags",
        "p_align",
    ]);
    for phdr in elf_file.segments() {
        let cells: Vec<Cell> = vec![
            elf::to_str::p_type_to_string(phdr.p_type).into(),
            format!("{:#x}", phdr.p_offset).into(),
            format!("{:#x}", phdr.p_vaddr).into(),
            format!("{:#x}", phdr.p_paddr).into(),
            format!("{:#x}", phdr.p_filesz).into(),
            format!("{:#x}", phdr.p_memsz).into(),
            phdr.p_align.into(),
            elf::to_str::p_flags_to_string(phdr.p_flags).into(),
        ];

        table.add_row(cells);
    }

    println!("{table}");
}



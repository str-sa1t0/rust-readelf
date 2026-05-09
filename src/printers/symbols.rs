use comfy_table::{Cell, Table};

use crate::types::ElfFile;

pub fn print_symbol_table(elf_file: &mut ElfFile) {
    let (symtab, strtab) = match elf_file
        .symbol_table()
        .expect("Failed to get .symtab and string table")
    {
        Some(tables) => tables,
        None => return,
    };

    let mut table = Table::new();

    table.set_header([
        "ndx",
        "value",
        "size",
        "type",
        "bind",
        "visibility",
        "shndx",
        "name",
    ]);

    for (ndx, sym) in symtab.iter().enumerate() {
        let name = strtab
            .get(sym.st_name as usize)
            .expect("Failed to get name from string table");

        let cells: Vec<Cell> = vec![
            ndx.into(),
            format!("{:#x}", sym.st_value).into(),
            sym.st_size.into(),
            elf::to_str::st_symtype_to_string(sym.st_symtype()).into(),
            elf::to_str::st_bind_to_string(sym.st_bind()).into(),
            elf::to_str::st_vis_to_string(sym.st_vis()).into(),
            sym.st_shndx.into(),
            name.into(),
        ];

        table.add_row(cells);
    }

    println!("{table}");
}

pub fn print_dynamic_symbol_table(elf_file: &mut ElfFile) {
    let (dynsyms, dynstrs) = match elf_file
        .dynamic_symbol_table()
        .expect("Failed to get .dynsym and string table")
    {
        Some(tables) => tables,
        None => return,
    };

    let symbols: Vec<(String, elf::symbol::Symbol)> = dynsyms
        .iter()
        .map(|sym| {
            (
                dynstrs
                    .get(sym.st_name as usize)
                    .expect("Failed to get symbol name")
                    .to_string(),
                sym,
            )
        })
        .collect();

    let vertab = elf_file
        .symbol_version_table()
        .expect("Failed to parse GNU symbol versions");

    let mut table = Table::new();

    table.set_header([
        "ndx",
        "value",
        "size",
        "type",
        "bind",
        "visibility",
        "shndx",
        "needs version",
        "name",
    ]);

    for (sym_idx, (sym_name, sym)) in symbols.iter().enumerate() {
        let needs_name = match &vertab {
            Some(vertab) => {
                if sym.is_undefined() {
                    match vertab
                        .get_requirement(sym_idx)
                        .expect("Failed to parse symbol requirement")
                    {
                        Some(req) => req.name,
                        None => "None",
                    }
                } else {
                    "None"
                }
            }
            None => "None",
        };

        let cells: Vec<Cell> = vec![
            sym_idx.into(),
            format!("{:#x}", sym.st_value).into(),
            sym.st_size.into(),
            elf::to_str::st_symtype_to_string(sym.st_symtype()).into(),
            elf::to_str::st_bind_to_string(sym.st_bind()).into(),
            elf::to_str::st_vis_to_string(sym.st_vis()).into(),
            sym.st_shndx.into(),
            needs_name.into(),
            sym_name.into(),
        ];

        table.add_row(cells);
    }

    println!("{table}");
}
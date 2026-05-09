use comfy_table::{Cell, Table};

use crate::types::ElfFile;

pub fn print(elf_file: &mut ElfFile) {
    let (shdrs, strtab) = elf_file
        .section_headers_with_strtab()
        .expect("Failed to read section table and string table");

    let (shdrs, strtab) = (shdrs, strtab.unwrap());

    let mut table = Table::new();

    table.set_header([
        "index",
        "name",
        "sh_type",
        "sh_addr",
        "sh_offset",
        "sh_size",
        "sh_entsize",
        "sh_flags",
        "sh_link",
        "sh_info",
        "sh_addralign",
    ]);

    for (ndx, shdr) in shdrs.iter().enumerate() {
        let name = strtab
            .get(shdr.sh_name as usize)
            .expect("Failed to get name from string table");

        let cells: Vec<Cell> = vec![
            ndx.into(),
            name.into(),
            elf::to_str::sh_type_to_string(shdr.sh_type).into(),
            format!("{:#x}", shdr.sh_addr).into(),
            format!("{:#x}", shdr.sh_offset).into(),
            format!("{:#x}", shdr.sh_size).into(),
            shdr.sh_entsize.into(),
            format!("{:#x}", shdr.sh_flags).into(),
            shdr.sh_link.into(),
            shdr.sh_info.into(),
            shdr.sh_addralign.into(),
        ];

        table.add_row(cells);
    }

    println!("{table}");
}
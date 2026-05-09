use comfy_table::{Cell, Table};

use crate::types::ElfFile;

pub fn print(elf_file: &mut ElfFile) {
    let dyns = match elf_file.dynamic().expect("Failed to get .dynamic") {
        Some(dyns) => dyns,
        None => return,
    };

    let mut table = Table::new();

    table.set_header(["d_tag", "d_ptr/d_val"]);

    for d in dyns.iter() {
        let d_tag_str = elf::to_str::d_tag_to_str(d.d_tag)
            .map_or(format!("{:#X?}", d.d_tag), |val| val.to_string());

        let cells: Vec<Cell> = vec![
            d_tag_str.into(),
            format!("{:#X?}", d.d_val()).into(),
        ];

        table.add_row(cells);
    }

    println!("{table}");
}
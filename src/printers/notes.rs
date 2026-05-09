use comfy_table::{Cell, Table};
use elf::note::Note;

use crate::types::ElfFile;

pub fn print(elf_file: &mut ElfFile) {
    let (shdrs, strtab) = elf_file
        .section_headers_with_strtab()
        .expect("strtab should be readable");

    let (shdrs, strtab) = (shdrs, strtab.unwrap());

    let shdrs_with_names: Vec<_> = shdrs
        .iter()
        .filter(|shdr| shdr.sh_type == elf::abi::SHT_NOTE)
        .map(|shdr| {
            let name = strtab
                .get(shdr.sh_name as usize)
                .expect("section name should parse");

            (name.to_string(), *shdr)
        })
        .collect();

    for (sh_name, shdr) in shdrs_with_names {
        let notes = elf_file
            .section_data_as_notes(&shdr)
            .expect("Failed to read notes section");

        println!("Displaying notes found in: {sh_name}");

        for note in notes {
            match note {
                Note::GnuAbiTag(abi) => {
                    let os_str = elf::to_str::note_abi_tag_os_to_str(abi.os)
                        .map_or(format!("{}", abi.os), |val| val.to_string());

                    println!(
                        "    OS: {os_str}, ABI: {}.{}.{}",
                        abi.major, abi.minor, abi.subminor
                    );
                }

                Note::GnuBuildId(build_id) => {
                    print!("    Build ID: ");

                    for byte in build_id.0 {
                        print!("{byte:02x}");
                    }

                    println!();
                }

                Note::Unknown(any) => {
                    let mut table = Table::new();

                    table.set_header(["type", "name", "desc"]);

                    let cells: Vec<Cell> = vec![
                        any.n_type.into(),
                        format!("{:?}", any.name).into(),
                        format!("{:02X?}", any.desc).into(),
                    ];

                    table.add_row(cells);

                    println!("{table}");
                }
            }
        }
    }
}
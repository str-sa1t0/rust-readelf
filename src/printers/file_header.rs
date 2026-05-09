use elf::endian::AnyEndian;
use elf::file::FileHeader;
use elf::to_str::{
    e_machine_to_human_str,
    e_osabi_to_string,
    e_type_to_human_str,
};

pub fn print(ehdr: &FileHeader<AnyEndian>) {
    let e_type_str = match e_type_to_human_str(ehdr.e_type) {
        Some(s) => s.to_string(),
        None => format!("e_type({:#x})", ehdr.e_type),
    };

    let e_machine_str = match e_machine_to_human_str(ehdr.e_machine) {
        Some(s) => s.to_string(),
        None => format!("e_machine({:#x})", ehdr.e_machine),
    };

    println!("File Header:");
    println!("  Class: {:?}", ehdr.class);
    println!("  Endianness: {:?}", ehdr.endianness);
    println!("  Version: {:?}", ehdr.version);
    println!("  OSABI: {}", e_osabi_to_string(ehdr.osabi));
    println!("  ABI Version: {}", ehdr.abiversion);
    println!("  Object Type: {e_type_str}");
    println!("  Arch: {e_machine_str}");
    println!("  Entry point address: {:#x}", ehdr.e_entry);

    println!(
        "  Start of program headers: {:#x} (bytes into file)",
        ehdr.e_phoff
    );

    println!(
        "  Start of section headers: {:#x} (bytes into file)",
        ehdr.e_shoff
    );

    println!("  Flags: {:#x}", ehdr.e_flags);
    println!("  Size of this header: {:#x}", ehdr.e_ehsize);
    println!("  Size of program header: {:#x}", ehdr.e_phentsize);
    println!("  Number of program headers: {:#x}", ehdr.e_phnum);
    println!("  Size of section header: {:#x}", ehdr.e_shentsize);
    println!("  Number of section headers: {:#x}", ehdr.e_shnum);

    println!(
        "  Section headers string table section index: {}",
        ehdr.e_shstrndx
    );
}
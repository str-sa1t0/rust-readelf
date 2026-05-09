use elf::endian::AnyEndian;
use elf::ElfStream;

pub type ElfFile = ElfStream<AnyEndian, std::fs::File>;
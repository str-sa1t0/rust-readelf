use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub file_name: String,

    #[arg(long)]
    pub file_header: bool,

    #[arg(long)]
    pub program_headers: bool,

    #[arg(long)]
    pub section_headers: bool,

    #[arg(long)]
    pub symbols: bool,

    #[arg(long)]
    pub dynamic_symbols: bool,

    #[arg(long)]
    pub dynamic: bool,

    #[arg(long)]
    pub relocations: bool,

    #[arg(long)]
    pub notes: bool,
}
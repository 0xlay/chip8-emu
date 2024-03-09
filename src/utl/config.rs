use clap_derive::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(long)]
    pub rom_path: String,
    #[arg(long, default_value_t = 800)]
    pub width: u32,
    #[arg(long, default_value_t = 600)]
    pub height: u32,
}

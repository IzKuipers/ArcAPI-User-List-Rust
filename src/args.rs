use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub server: String,
    #[arg(short, long, default_value = "")]
    pub authcode: String,
}

pub fn parse_args() -> Args {
    let args = Args::parse();

    return args;
}

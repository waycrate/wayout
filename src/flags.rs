use clap::Parser;

#[derive(Parser)]
#[command(
    name = "wayout",
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = "Simple tool to set output mode of Heads for wlroots based compositors."
)]
pub struct CLI {
    #[arg(long, value_name = "OUTPUT")]
    pub on: Option<String>,

    #[arg(long, value_name = "OUTPUT")]
    pub off: Option<String>,

    #[arg(long, value_name = "OUTPUT")]
    pub toggle: Option<String>,
}

pub fn parse_flags() -> CLI {
    return CLI::parse();
}

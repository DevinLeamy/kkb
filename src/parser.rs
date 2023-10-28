use clap::*;

#[derive(Parser, Debug)]
pub struct Arguments {
    pub prompt: Option<String>,
    #[arg(short, long)]
    pub output: Option<String>,
}

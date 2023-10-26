use clap::*;

#[derive(Parser, Debug)]
pub struct Arguments {
    pub prompt: Option<String>,
}

use clap::*;

#[derive(Parser, Debug)]
pub struct Arguments {
    prompt: Option<String>,
}

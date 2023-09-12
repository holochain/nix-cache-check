
use app::{App, run_app};
use clap::Parser;

mod parser;
mod app;

fn main() -> anyhow::Result<()> {
    let args = App::parse();

    run_app(args)
}

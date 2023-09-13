
use app::run_app;

mod parser;
mod app;

fn main() -> anyhow::Result<()> {
    println!("Starting app");
    run_app()
}

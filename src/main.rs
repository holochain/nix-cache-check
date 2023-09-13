
use app::run_app;

mod parser;
mod app;

fn main() -> anyhow::Result<()> {
    run_app()
}

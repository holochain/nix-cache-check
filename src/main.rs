use app::run_app;

mod app;
mod parser;

fn main() -> anyhow::Result<()> {
    run_app()
}

#[derive(clap::Parser, Debug)]
pub struct Args {
    query: String,
    limit: usize,
}

#[derive(clap::Parser, Debug)]
pub struct Args {
    query: Option<String>,
    limit: Option<usize>,
    id: Option<String>,
}

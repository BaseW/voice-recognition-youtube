use recognition_from_link::{all, download, recognize, search};

#[derive(clap::Subcommand, Debug)]
enum Command {
    /// search videos only
    Search(search::Args),
    /// download video only
    Download(download::Args),
    /// recognize video only
    Recognize(recognize::Args),
    /// search, download, recognize
    All(all::Args),
}

fn parse_level(s: &str) -> anyhow::Result<log::LevelFilter> {
    s.parse::<log::LevelFilter>()
        .map_err(|_| anyhow::anyhow!("failed to parse level '{s}'"))
}

#[derive(Debug, clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Opts {
    /// The log level for messages, only log messages at or above
    /// the level will be emitted.
    #[clap(
        short = 'L',
        default_value = "warn",
        value_parser = parse_level,
        long_help = "The log level for messages, only log messages at or above the level will be emitted.
Possible values:
* off
* error
* warn
* info
* debug
* trace"
    )]
    log_level: log::LevelFilter,
    #[clap(subcommand)]
    cmd: Command,
}

#[tokio::main]
async fn main() {
    use clap::Parser;

    let args = Opts::parse();

    match args.cmd {
        Command::Search(args) => {
            search::search(args.query, args.limit).await;
        }
        Command::Download(args) => {
            download::download(args.query, args.limit, args.id)
                .await
                .unwrap();
        }
        Command::Recognize(args) => {
            recognize::recognize(&args.id, &args.mode).await;
        }
        Command::All(args) => {
            all::all(args.query, args.limit, args.id, args.mode).await;
        }
    }
}

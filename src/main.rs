mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

fn main() {
    if let Err(err) = args::Config::new().and_then(args::Config::run) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

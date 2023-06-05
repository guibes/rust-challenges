use clap::Parser;
use std::env;
use std::path::Path;

const EMPTY_HOME_DIR: &str = "";
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = EMPTY_HOME_DIR.to_string())]
    dir: String,
}

fn main() {
    let args = Args::parse();
    let dir;

    if args.dir == "" {
        dir = std::env::var("HOME").unwrap_or("none".to_string());
    } else {
        dir = args.dir;
    }

    let root = Path::new(&dir);
    assert!(env::set_current_dir(&root).is_ok());
    println!(
        "Successfully changed working directory to {}!",
        root.display()
    );
}

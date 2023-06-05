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
    let dir = get_home_dir(&args.dir);
    let root = Path::new(&dir);
    assert!(env::set_current_dir(&root).is_ok());
    println!(
        "Successfully changed working directory to {}!",
        root.display()
    );
}

fn get_home_dir(dir: &str) -> String {
    if dir != "" {
        return dir.to_string();
    }
    let home_dir = std::env::var("HOME").unwrap_or("~".to_string());
    return home_dir;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_home_dir() {
        let test_dir = "/home/username";
        let dir = get_home_dir(&test_dir);
        assert_eq!(dir, test_dir.to_string());
    }

    #[test]
    fn test_get_home_dir_empty() {
        let dir = get_home_dir("");
        let home_dir = std::env::var("HOME").unwrap_or("~".to_string());
        assert_eq!(dir, home_dir);
    }
}

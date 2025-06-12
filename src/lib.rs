use std::error::Error;
use std::fs;
use std::path::PathBuf;
use clap::Parser;
use regex::Regex;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config {
    /// Query string to search for
    #[arg(short, long)]
    pub query: String,

    /// File or directory path
    #[arg(short, long)]
    pub path: PathBuf,

    /// Ignore case
    #[arg(short, long)]
    pub ignore_case: bool,

    /// Show summary only
    #[arg(short, long)]
    pub summary: bool,
}

impl Config {
    pub fn build() -> Result<Self, Box<dyn Error>> {
        Ok(Config::parse())
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let matcher = Regex::new(&config.query)?;
    let paths = collect_paths(&config.path);

    let mut match_count = 0;
    for path in paths {
        let contents = fs::read_to_string(&path)?;
        for (i, line) in contents.lines().enumerate() {
            let haystack = if config.ignore_case {
                line.to_lowercase()
            } else {
                line.to_string()
            };
            let needle = if config.ignore_case {
                config.query.to_lowercase()
            } else {
                config.query.clone()
            };

            if matcher.is_match(&haystack) {
                match_count += 1;
                if !config.summary {
                    println!("{}:{}: {}", path.display(), i + 1, line);
                }
            }
        }
    }

    if config.summary {
        println!("Total matches: {}", match_count);
    }

    Ok(())
}

fn collect_paths(path: &PathBuf) -> Vec<PathBuf> {
    if path.is_file() {
        vec![path.clone()]
    } else {
        WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.path().to_path_buf())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_regex_match() {
        let re = Regex::new("duct").unwrap();
        let text = "Rust:
safe, fast, productive.
Duct tape.";
        let matches: Vec<&str> = text.lines()
            .filter(|line| re.is_match(line))
            .collect();

        assert_eq!(matches, vec!["safe, fast, productive."]);
    }

    #[test]
    fn case_insensitive_match() {
        let re = Regex::new("rust").unwrap();
        let text = "Rust:
safe, fast, productive.
Trust me.";
        let matches: Vec<&str> = text.lines()
            .filter(|line| re.is_match(&line.to_lowercase()))
            .collect();

        assert_eq!(matches, vec!["Rust:", "Trust me."]);
    }
}

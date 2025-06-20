use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use serde::Serialize;

/// Command-line arguments parser using `clap`
#[derive(Parser)]
#[command(name = "File Analyzer", version, about = "Analyze or search a text file")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze a file for word stats
    Analyze {
        #[arg(short, long)]
        file: PathBuf,
        #[arg(long)]
        json: bool,
    },

    /// Search for a word and show matching lines
    Search {
        #[arg(short, long)]
        file: PathBuf,
        #[arg(short, long)]
        word: String,
    },
}

/// Output data structure
#[derive(Serialize)]
struct AnalysisResult {
    characters: usize,
    words: usize,
    lines: usize,
    longest_word: String,
    average_word_length: f32,
}

fn analyze_text(content: &str) -> AnalysisResult {
    let characters = content.chars().count();
    let lines = content.lines().count();
    let words_vec: Vec<&str> = content.split_whitespace().collect();
    let words = words_vec.len();

    let longest_word = words_vec.iter()
        .max_by_key(|w| w.len())
        .unwrap_or(&"")
        .to_string();

    let avg_word_len = if words == 0 {
        0.0
    } else {
        words_vec.iter().map(|w| w.len()).sum::<usize>() as f32 / words as f32
    };

    AnalysisResult {
        characters,
        words,
        lines,
        longest_word,
        average_word_length: avg_word_len,
    }
}

fn search_word(content: &str, target: &str) -> Vec<(usize, String)> {
    content
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            if line.to_lowercase().contains(&target.to_lowercase()) {
                Some((i + 1, line.to_string()))
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Analyze { file, json } => {
            let contents = fs::read_to_string(&file).unwrap_or_else(|_| {
                eprintln!("❌ Could not read file '{}'", file.display());
                std::process::exit(1);
            });

            let analysis = analyze_text(&contents);

            if json {
                let json = serde_json::to_string_pretty(&analysis).unwrap();
                println!("{}", json);
            } else {
                println!("📊 Analysis of '{}'", file.display());
                println!("Characters        : {}", analysis.characters);
                println!("Words             : {}", analysis.words);
                println!("Lines             : {}", analysis.lines);
                println!("Longest word      : {}", analysis.longest_word);
                println!("Average word len  : {:.2}", analysis.average_word_length);
            }
        }

        Commands::Search { file, word } => {
            let contents = fs::read_to_string(&file).unwrap_or_else(|_| {
                eprintln!("❌ Could not read file '{}'", file.display());
                std::process::exit(1);
            });

            let results = search_word(&contents, &word);

            if results.is_empty() {
                println!("🔍 No occurrences of '{}' found.", word);
            } else {
                println!("🔍 Found '{}' in the following lines:", word);
                for (line_num, line) in results {
                    println!("Line {}: {}", line_num, line);
                }
            }
        }
    }
}

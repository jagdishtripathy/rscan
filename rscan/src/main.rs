use regex::Regex;
use walkdir::WalkDir;
use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::path::Path;

fn print_banner() {
    println!(
        r#"
██████╗ ███████╗ ██████╗ █████╗ ███╗   ██╗
██╔══██╗██╔════╝██╔════╝██╔══██╗████╗  ██║
██████╔╝█████╗  ██║     ███████║██╔██╗ ██║
██╔═══╝ ██╔══╝  ██║     ██╔══██║██║╚██╗██║
██║     ███████╗╚██████╗██║  ██║██║ ╚████║
╚═╝     ╚══════╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═══╝
         🔍  Rust Secret Scanner - rscan
"#
    );
}

fn main() {
    print_banner(); // show banner first

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <directory_path>", args[0]);
        std::process::exit(1);
    }

    let dir_path = &args[1];

    // Define regex patterns for secrets
    let patterns = vec![
        ("AWS Access Key", r"AKIA[0-9A-Z]{16}"),
        ("AWS Secret Key", r#"(?i)aws(.{0,20})?(secret|private)?(.{0,20})?['"][0-9a-zA-Z/+]{40}['"]"#),
        ("JWT Token", r#"eyJ[A-Za-z0-9-_]+\.[A-Za-z0-9-_]+\.[A-Za-z0-9-_]+"#),
        ("Generic API Key", r#"api[_-]?key['"]?\s*[:=]\s*['"][a-zA-Z0-9\-_.]+['"]"#),
        ("Password in text", r#"password\s*[:=]\s*['"][^'"]+['"]"#),
    ];

    let regexes: Vec<(&str, Regex)> = patterns
        .iter()
        .map(|(name, pat)| (*name, Regex::new(pat).unwrap()))
        .collect();

    for entry in WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        // Allow only specific file types
        if let Some(ext) = path.extension() {
            if !["txt", "log", "json", "env", "config", "ini"]
                .contains(&ext.to_str().unwrap_or(""))
            {
                continue;
            }
        } else {
            continue;
        }

        if let Err(err) = scan_file(path, &regexes) {
            eprintln!("Error reading file {:?}: {}", path, err);
        }
    }
}

fn scan_file(path: &Path, regexes: &[(&str, Regex)]) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for (num, line) in reader.lines().enumerate() {
        let line = line?;
        for (name, re) in regexes.iter() {
            if re.is_match(&line) {
                println!("🔐 Found [{}] in file {:?} at line {}: {}", name, path, num + 1, line.trim());
            }
        }
    }

    Ok(())
}

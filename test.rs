use std::process::{Command, exit};

fn main() {
    if let Ok(output) = Command::new("git").arg("log").arg("--shortstat").output() {
        if output.status.success() {
            let log_output = String::from_utf8_lossy(&output.stdout);
            display_commit_stats(&log_output);
        } else {
            eprintln!("Failed to retrieve git log: {:?}", output);
        }
    } else {
        eprintln!("Error running git log");
        exit(-1);
    }
}

fn display_commit_stats(log_output: &str) {
    let mut current_author = String::new();
    let mut lines_added = 0;
    let mut lines_removed = 0;

    for line in log_output.lines() {
        if line.starts_with("Author:") {
            current_author = line.trim_start_matches("Author:").trim().to_string();
        } else if let Some((added, removed)) = extract_numbers(line) {
            lines_added += added;
            lines_removed += removed;
        } else if line.is_empty() {
            if !current_author.is_empty() {
                println!(
                    "Commit by: {} LOC Added: {} LOC Removed: {}",
                    current_author, lines_added, lines_removed
                );
            }
            current_author.clear();
            lines_added = 0;
            lines_removed = 0;
        }
    }
}
fn extract_numbers(line: &str) -> Option<(u32, u32)> {
    if line.contains("+") && line.contains("-") {
        let splitted: Vec<&str> = line.split(" ").into_iter().collect();
        return Some((splitted.get(4).unwrap().to_string().parse().unwrap(), splitted.get(6).unwrap().to_string().parse().unwrap()));
    }
    None
    
}

use super::parsing::parse_node;
use crate::core::{DEPTH_0_SENTINEL, validate_node};
use crate::tablebase::TablebaseIndex;
use std::io::{self, BufRead, Write};

use super::analyze::analyze;

pub fn run(tablebase: Option<TablebaseIndex>) {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                break;
            }
        };

        let parts = parse_command_line(line.trim());

        match parts.first().map(|s| s.as_str()) {
            Some("version") => {
                writeln!(stdout, "{}", env!("CARGO_PKG_VERSION")).unwrap();
                stdout.flush().unwrap();
            }
            Some("info") => {
                handle_info(&tablebase, &mut stdout);
            }
            Some("analyze") => {
                handle_analyze(&parts, &tablebase, &mut stdout);
            }
            Some("quit") => {
                break;
            }
            Some(cmd) => {
                writeln!(stdout, "error Unknown command: {}", cmd).unwrap();
                stdout.flush().unwrap();
            }
            None => {
                // Empty line, ignore
            }
        }
    }
}

fn parse_command_line(input: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for ch in input.chars() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
            }
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                current.push(ch);
            }
        }
    }

    if !current.is_empty() {
        parts.push(current);
    }

    parts
}

fn handle_analyze(parts: &[String], tablebase: &Option<TablebaseIndex>, stdout: &mut impl Write) {
    let mut errors = Vec::new();

    // Parse command: analyze <ply_string|"root">
    if parts.len() < 2 {
        errors.push("analyze requires a ply string or 'root'".to_string());
        output_errors(stdout, &errors);
        return;
    }

    let ply_string = &parts[1];

    // Parse the node
    let node = if ply_string == "root" {
        DEPTH_0_SENTINEL
    } else {
        // Try to parse the node
        let parsed_node = match parse_node(ply_string) {
            Ok(p) => p,
            Err(e) => {
                errors.push(format!("{}", e));
                output_errors(stdout, &errors);
                return;
            }
        };

        // Try to validate the node
        if let Err(e) = validate_node(&parsed_node) {
            errors.push(format!("{}", e));
            output_errors(stdout, &errors);
            return;
        }

        parsed_node
    };

    // Analyze and output JSON
    let json = analyze(&node, tablebase.as_ref());
    writeln!(stdout, "{}", json).unwrap();
    stdout.flush().unwrap();
}

fn handle_info(tablebase: &Option<TablebaseIndex>, stdout: &mut impl Write) {
    use serde_json::json;

    let info = if let Some(tb) = tablebase {
        let available = tb.available_depths();
        let memory_bytes = tb.memory_usage();

        // Collect loaded depths
        let loaded_depths: Vec<usize> = available
            .iter()
            .enumerate()
            .filter_map(|(idx, &loaded)| if loaded { Some(idx) } else { None })
            .collect();

        json!({
            "loaded": true,
            "depths": loaded_depths,
            "memory_bytes": memory_bytes
        })
    } else {
        json!({
            "loaded": false
        })
    };

    writeln!(stdout, "{}", info).unwrap();
    stdout.flush().unwrap();
}

fn output_errors(stdout: &mut impl Write, errors: &[String]) {
    if let Ok(json) = serde_json::to_string(&errors) {
        writeln!(stdout, "{}", json).unwrap();
        stdout.flush().unwrap();
    }
}

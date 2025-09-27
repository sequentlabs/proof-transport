// tests/support.rs
use proof_transport::ast::Proof;
use std::fs;
use std::path::{Path, PathBuf};

/// Recursively collect `*.json` example files under `examples/`.
pub fn example_paths() -> Vec<PathBuf> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        if let Ok(rd) = fs::read_dir(dir) {
            for entry in rd.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    walk(&p, out);
                } else if p.extension().map_or(false, |e| e == "json") {
                    out.push(p);
                }
            }
        }
    }
    let mut v = Vec::new();
    walk(Path::new("examples"), &mut v);
    v.sort();
    v
}

/// Convert “JSON‑ish” (comments, trailing commas) into strict JSON.
/// * Strips `//` and `/* ... */` comments (not inside strings)
/// * Removes trailing commas before `]` / `}` (not inside strings)
fn to_strict_json(src: &str) -> String {
    // Pass 1: strip comments
    let mut no_comments = String::with_capacity(src.len());
    let bytes = src.as_bytes();
    let mut i = 0;
    let (mut in_str, mut esc) = (false, false);
    let (mut in_line_comment, mut in_block_comment) = (false, false);

    while i < bytes.len() {
        let c = bytes[i] as char;

        if in_line_comment {
            if c == '\n' {
                in_line_comment = false;
                no_comments.push('\n');
            }
            i += 1;
            continue;
        }
        if in_block_comment {
            if c == '*' && i + 1 < bytes.len() && bytes[i + 1] as char == '/' {
                in_block_comment = false;
                i += 2;
            } else {
                i += 1;
            }
            continue;
        }
        if in_str {
            no_comments.push(c);
            if esc {
                esc = false;
            } else if c == '\\' {
                esc = true;
            } else if c == '"' {
                in_str = false;
            }
            i += 1;
            continue;
        }

        // Not in string or comment
        if c == '"' {
            in_str = true;
            no_comments.push(c);
            i += 1;
            continue;
        }
        if c == '/' && i + 1 < bytes.len() {
            let c2 = bytes[i + 1] as char;
            if c2 == '/' {
                in_line_comment = true;
                i += 2;
                continue;
            }
            if c2 == '*' {
                in_block_comment = true;
                i += 2;
                continue;
            }
        }

        no_comments.push(c);
        i += 1;
    }

    // Pass 2: remove trailing commas
    let mut out = String::with_capacity(no_comments.len());
    let b = no_comments.as_bytes();
    let mut j = 0;
    in_str = false;
    esc = false;

    while j < b.len() {
        let ch = b[j] as char;

        if in_str {
            out.push(ch);
            if esc {
                esc = false;
            } else if ch == '\\' {
                esc = true;
            } else if ch == '"' {
                in_str = false;
            }
            j += 1;
            continue;
        }

        if ch == '"' {
            in_str = true;
            out.push(ch);
            j += 1;
            continue;
        }

        if ch == ',' {
            // Look ahead to next non‑whitespace char
            let mut k = j + 1;
            while k < b.len() && (b[k] as char).is_whitespace() {
                k += 1;
            }
            if k < b.len() {
                let next = b[k] as char;
                if next == ']' || next == '}' {
                    j += 1; // drop trailing comma
                    continue;
                }
            }
        }

        out.push(ch);
        j += 1;
    }

    out
}

/// Parse a proof from a file, tolerating comments/trailing commas in fixtures.
pub fn parse_proof(path: &Path) -> Result<Proof, serde_json::Error> {
    let s = fs::read_to_string(path).expect("read example file");

    // Try strict JSON first
    match serde_json::from_str::<Proof>(&s) {
        Ok(p) => Ok(p),
        Err(e_strict) => {
            // Sanitize, then re‑try strict JSON
            let cleaned = to_strict_json(&s);
            match serde_json::from_str::<Proof>(&cleaned) {
                Ok(p) => Ok(p),
                Err(_e_again) => Err(e_strict), // propagate the original serde_json error
            }
        }
    }
}

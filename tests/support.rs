// tests/support.rs
use proof_transport::ast::Proof;
use std::fs;
use std::path::{Path, PathBuf};

/// Recursively collect `*.json` example files under `examples/`.
#[allow(dead_code)]
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

/// Convert “JSON‑ish” into strict JSON (strip comments + trailing commas).
fn to_strict_json(src: &str) -> String {
    // Pass 1: strip // and /* */ comments (not inside strings).
    let mut no_comments = String::with_capacity(src.len());
    let b = src.as_bytes();
    let (mut i, mut in_str, mut esc) = (0, false, false);
    let (mut in_line_comment, mut in_block_comment) = (false, false);

    while i < b.len() {
        let c = b[i] as char;
        if in_line_comment {
            if c == '\n' {
                in_line_comment = false;
                no_comments.push('\n');
            }
            i += 1;
            continue;
        }
        if in_block_comment {
            if c == '*' && i + 1 < b.len() && b[i + 1] as char == '/' {
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
        if c == '"' {
            in_str = true;
            no_comments.push(c);
            i += 1;
            continue;
        }
        if c == '/' && i + 1 < b.len() {
            let c2 = b[i + 1] as char;
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

    // Pass 2: remove trailing commas outside strings.
    let mut out = String::with_capacity(no_comments.len());
    let b2 = no_comments.as_bytes();
    let (mut j, mut in_str2, mut esc2) = (0, false, false);

    while j < b2.len() {
        let ch = b2[j] as char;
        if in_str2 {
            out.push(ch);
            if esc2 {
                esc2 = false;
            } else if ch == '\\' {
                esc2 = true;
            } else if ch == '"' {
                in_str2 = false;
            }
            j += 1;
            continue;
        }
        if ch == '"' {
            in_str2 = true;
            out.push(ch);
            j += 1;
            continue;
        }
        if ch == ',' {
            let mut k = j + 1;
            while k < b2.len() && (b2[k] as char).is_whitespace() {
                k += 1;
            }
            if k < b2.len() {
                let next = b2[k] as char;
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
    match serde_json::from_str::<Proof>(&s) {
        Ok(p) => Ok(p),
        Err(e_strict) => {
            let cleaned = to_strict_json(&s);
            match serde_json::from_str::<Proof>(&cleaned) {
                Ok(p) => Ok(p),
                Err(_e_again) => Err(e_strict),
            }
        }
    }
}

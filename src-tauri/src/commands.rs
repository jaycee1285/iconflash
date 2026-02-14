use serde::Serialize;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Serialize)]
pub struct SvgFile {
    path: String,
    size: u64,
    content: String,
}

#[derive(Serialize)]
pub struct ScanResult {
    source_dir: String,
    preview_svgs: Vec<SvgFile>,
    total_svg_count: usize,
    non_svg_count: usize,
}

#[derive(Serialize)]
pub struct ExportResult {
    output_dir: String,
    svgs_processed: usize,
    files_copied: usize,
}

#[tauri::command]
pub fn scan_directory(path: String) -> Result<ScanResult, String> {
    let root = Path::new(&path);
    if !root.is_dir() {
        return Err(format!("'{}' is not a directory", path));
    }

    let mut seen_canonical: HashSet<PathBuf> = HashSet::new();
    let mut svg_entries: Vec<(PathBuf, u64)> = Vec::new();
    let mut non_svg_count: usize = 0;

    for entry in WalkDir::new(root).follow_links(true) {
        let entry = entry.map_err(|e| e.to_string())?;
        let file_path = entry.path();

        if entry.file_type().is_dir() {
            continue;
        }

        let is_svg = file_path
            .extension()
            .map_or(false, |e| e.eq_ignore_ascii_case("svg"));

        if !is_svg {
            non_svg_count += 1;
            continue;
        }

        let canonical = fs::canonicalize(file_path).map_err(|e| e.to_string())?;
        if !seen_canonical.insert(canonical) {
            continue;
        }

        let metadata = fs::metadata(file_path).map_err(|e| e.to_string())?;
        svg_entries.push((file_path.to_path_buf(), metadata.len()));
    }

    let total_svg_count = svg_entries.len();

    svg_entries.sort_by(|a, b| b.1.cmp(&a.1));
    svg_entries.truncate(5);

    let preview_svgs: Vec<SvgFile> = svg_entries
        .into_iter()
        .filter_map(|(p, size)| {
            let content = fs::read_to_string(&p).ok()?;
            Some(SvgFile {
                path: p.to_string_lossy().to_string(),
                size,
                content,
            })
        })
        .collect();

    Ok(ScanResult {
        source_dir: path,
        preview_svgs,
        total_svg_count,
        non_svg_count,
    })
}

fn replace_color_insensitive(content: &str, from: &str, to: &str) -> String {
    let lower_content = content.to_lowercase();
    let from_lower = from.to_lowercase();
    let from_len = from.len();
    let mut result = String::with_capacity(content.len());
    let mut last = 0;

    for (idx, _) in lower_content.match_indices(&from_lower) {
        // Check that the match is not followed by another hex digit
        let after = idx + from_len;
        if after < content.len() {
            let next_byte = content.as_bytes()[after];
            if next_byte.is_ascii_hexdigit() {
                continue;
            }
        }
        result.push_str(&content[last..idx]);
        result.push_str(to);
        last = after;
    }
    result.push_str(&content[last..]);
    result
}

fn to_short_hex(hex: &str) -> Option<String> {
    let bytes = hex.as_bytes();
    if bytes.len() == 7 && bytes[1] == bytes[2] && bytes[3] == bytes[4] && bytes[5] == bytes[6] {
        Some(format!(
            "#{}{}{}",
            bytes[1] as char, bytes[3] as char, bytes[5] as char
        ))
    } else {
        None
    }
}

fn apply_color_mappings(content: &str, mappings: &[(String, String)]) -> String {
    let mut result = content.to_string();
    for (original, replacement) in mappings {
        // Replace 6-char form first
        result = replace_color_insensitive(&result, original, replacement);
        // If a 3-char short form exists, replace that too
        if let Some(short_original) = to_short_hex(original) {
            let short_replacement = to_short_hex(replacement).unwrap_or(replacement.clone());
            result = replace_color_insensitive(&result, &short_original, &short_replacement);
        }
    }
    result
}

#[tauri::command]
pub fn export_theme(
    source_dir: String,
    theme_name: String,
    color_mappings: Vec<(String, String)>,
) -> Result<ExportResult, String> {
    let home = dirs::home_dir().ok_or("Could not determine home directory")?;
    let output_dir = home.join(".local/share/icons").join(&theme_name);

    if output_dir.exists() {
        return Err(format!(
            "Theme '{}' already exists at {}",
            theme_name,
            output_dir.display()
        ));
    }

    fs::create_dir_all(&output_dir).map_err(|e| format!("Failed to create output dir: {}", e))?;

    let source = Path::new(&source_dir);
    let mut svgs_processed: usize = 0;
    let mut files_copied: usize = 0;

    for entry in WalkDir::new(source).follow_links(false) {
        let entry = entry.map_err(|e| e.to_string())?;
        let src_path = entry.path();
        let rel_path = src_path
            .strip_prefix(source)
            .map_err(|e| e.to_string())?;
        let dest_path = output_dir.join(rel_path);

        if entry.path_is_symlink() {
            let target = fs::read_link(src_path).map_err(|e| e.to_string())?;
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            #[cfg(unix)]
            std::os::unix::fs::symlink(&target, &dest_path).map_err(|e| e.to_string())?;
        } else if entry.file_type().is_dir() {
            fs::create_dir_all(&dest_path).map_err(|e| e.to_string())?;
        } else {
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }

            let is_svg = src_path
                .extension()
                .map_or(false, |e| e.eq_ignore_ascii_case("svg"));

            if is_svg {
                let content =
                    fs::read_to_string(src_path).map_err(|e| format!("Read error: {}", e))?;
                let replaced = apply_color_mappings(&content, &color_mappings);
                fs::write(&dest_path, replaced)
                    .map_err(|e| format!("Write error: {}", e))?;
                svgs_processed += 1;
            } else {
                fs::copy(src_path, &dest_path)
                    .map_err(|e| format!("Copy error: {}", e))?;
                files_copied += 1;
            }
        }
    }

    Ok(ExportResult {
        output_dir: output_dir.to_string_lossy().to_string(),
        svgs_processed,
        files_copied,
    })
}
